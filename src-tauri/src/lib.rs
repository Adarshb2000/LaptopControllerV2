// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod control;
use std::{collections::HashMap, fmt::format, hash::Hash, thread, time::{Duration, Instant}};

use control::{handle_mouse_move, handle_scroll};

mod input;
use enigo::{Enigo, Keyboard, Mouse, Settings};
use gilrs::{Button, Event, EventType, Gilrs};
use input::{get_all_axis_data, get_gamepad_button_mapping, get_keyboard_focus_input};
use serde::Serialize;
use tauri::{ipc::Channel, AppHandle, Manager, PhysicalPosition, PhysicalSize};

#[derive(Debug, Serialize, Clone)]
pub struct KeyboardType {
    left: &'static [char],
    right: &'static [char],
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum UiEvent {
    ShowToast(String),
    KeyboardMode(bool),
    KeyFocus((i32, i32)),
    SwitchKeyboard(usize),
    KeyPress,
    Init(&'static [KeyboardType]),
}

static KEYBOARDS: &'static [KeyboardType] = &[
    KeyboardType {
        // Primary (non-hold) layout: 16 letters total (8 per joystick)
        left: &['E', 'T', 'A', 'O', 'I', 'D', 'C', 'W'],
        right: &['N', 'S', 'H', 'R', 'L', 'U', 'M', 'F'],
    },
    KeyboardType {
        // Secondary (hold) layout: 10 letters total (split as 5 per joystick)
        left: &['G', 'Y', 'P', 'V', 'X', '0', '1', '2'],
        right: &['B', 'K', 'J', 'Q', 'Z', '3', '4', '5'],
    },
];

const LONG_PRESS_THRESHOLD: Duration = Duration::from_millis(500); 

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![initialize])
        .setup(|app| {
            let app_handle = app.handle();
            let window = app_handle.get_webview_window("main").unwrap();
            if let Ok(Some(monitor)) = window.current_monitor() {
                let monitor_size = monitor.size();

                
                window.set_size(PhysicalSize::new(monitor_size.width, monitor_size.height)).unwrap();
                window.set_position(PhysicalPosition::new(0, 0)).unwrap();
            }
            window.set_always_on_top(true).unwrap();
            window.set_decorations(false).unwrap();
            window.set_ignore_cursor_events(true).unwrap();
            window.set_skip_taskbar(true).unwrap();
            window.set_shadow(false).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn initialize(_app: AppHandle, on_event: Channel<UiEvent>) {
    dbg!("Initializing");
    let dispatch = move |event: UiEvent| {
        on_event.send(event).unwrap();
    };

    dispatch(UiEvent::Init(KEYBOARDS));

    let _ = thread::spawn(move || {
        gamepad_handler(dispatch);
    });
}

fn gamepad_handler(dispatch: impl Fn(UiEvent)) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut gilrs = Gilrs::new().unwrap();
    let mut remote_active = false;
    let mut keyboard_mode = false;
    let mut active_keyboard: usize = 0;
    let mut last_state: (i32, i32) = (-1, -1);
    let mut active_key = (-1, -1);
    let poll_interval = Duration::from_millis(16);
    let mut press_times: HashMap<Button, Instant> = HashMap::new();
    
    loop {
        std::thread::sleep(poll_interval);
        while let Some(Event { event, .. }) = gilrs.next_event() {
            // Common button mapping
            match event {
                EventType::ButtonChanged(Button::Start, val, _) => {
                    if val == 1.0 {
                       press_times.insert(Button::Start, Instant::now()); 
                    } else {
                        if let Some(press_time) = press_times.remove(&Button::Start) {
                            let elapsed = press_time.elapsed();
                            if elapsed > LONG_PRESS_THRESHOLD {
                                if remote_active {
                                    keyboard_mode = false;
                                    dispatch(UiEvent::KeyboardMode(keyboard_mode));
                                }
                                remote_active = !remote_active;
                                dispatch(UiEvent::ShowToast(format!("Remote {}ctivated", if remote_active {"A"} else {"Dea"})));
                            } else if remote_active {
                                keyboard_mode = !keyboard_mode;
                                dispatch(UiEvent::KeyboardMode(keyboard_mode));
                                dispatch(UiEvent::ShowToast(format!("Keyboard mode {}ctivated", if keyboard_mode {"A"} else {"Dea"})));
                            }
                        }
                    }   
                }
                _ if !remote_active => {
                    dispatch(UiEvent::ShowToast("Remote inactive!".to_owned()));
                }
                EventType::ButtonChanged(Button::South, val, _) => {
                    if keyboard_mode {
                        let _ = val;
                    } else {
                        let _ = enigo.button(
                            enigo::Button::Left,
                            if val == 1.0 {
                                enigo::Direction::Press
                            } else {
                                enigo::Direction::Release
                            },
                        );
                    }
                }
                EventType::ButtonChanged(Button::LeftTrigger2, val, _) => {
                    if keyboard_mode {
                        active_keyboard = val as usize;
                        dispatch(UiEvent::SwitchKeyboard(active_keyboard));
                    }
                }
                EventType::ButtonPressed(Button::RightTrigger2, _) => {
                    if keyboard_mode && active_keyboard < KEYBOARDS.len() {
                        match active_key {
                            (-1, -1) => {
                                enigo.key(enigo::Key::Meta, enigo::Direction::Click).unwrap();
                            }
                            (-1, key) => {
                                let _ = enigo.text(
                                    &KEYBOARDS[active_keyboard].right[key as usize].to_string().to_ascii_lowercase(),
                                );
                                dispatch(UiEvent::KeyPress);
                            }
                            (key, -1) => {
                                let _ = enigo.text(
                                    &KEYBOARDS[active_keyboard].left[key as usize].to_string().to_ascii_lowercase(),
                                );
                                dispatch(UiEvent::KeyPress);
                            }
                            _ => {}
                        }
                    }
                }
                EventType::ButtonChanged(btn, val, _) => {
                    if let Some(button) = get_gamepad_button_mapping(btn, keyboard_mode) {
                        let _ = enigo.key(
                            button,
                            if val == 1.0 {
                                enigo::Direction::Press
                            } else {
                                enigo::Direction::Release
                            },
                        );
                    }
                }
                _ => {}
            }
        }

        if !remote_active {
            continue;
        }

        for (_, gamepad) in gilrs.gamepads() {
            if !gamepad.is_connected() {
                continue;
            }
            let (left_axis_data, right_axis_data) = get_all_axis_data(gamepad);
            if keyboard_mode {
                let focused_key = get_keyboard_focus_input(left_axis_data, right_axis_data);

                if focused_key == last_state {
                    break;
                } else if focused_key.0 != last_state.0 {
                    dispatch(UiEvent::KeyFocus((
                        focused_key.0,
                        if focused_key.0 == -1 {
                            focused_key.1
                        } else {
                            -1
                        },
                    )));
                    last_state.0 = focused_key.0;
                    active_key = (focused_key.0, -1);
                } else if focused_key.1 != last_state.1 {
                    dispatch(UiEvent::KeyFocus((
                        if focused_key.1 == -1 {
                            focused_key.0
                        } else {
                            -1
                        },
                        focused_key.1,
                    )));
                    last_state.1 = focused_key.1;
                    active_key = (-1, focused_key.1);
                }
            } else {
                handle_mouse_move(left_axis_data, &mut enigo);
                handle_scroll(right_axis_data, &mut enigo);
            }

            break;
        }
    }
}
