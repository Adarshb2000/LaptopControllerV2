use enigo::Key;
use gilrs::Button;

pub fn get_gamepad_button_mapping(button: Button, keyboard_mode: bool) -> Option<Key> {
    match button {
        // common
        Button::DPadDown => Some(Key::DownArrow),
        Button::DPadUp => Some(Key::UpArrow),
        Button::DPadLeft => Some(Key::LeftArrow),
        Button::DPadRight => Some(Key::RightArrow),
        Button::RightTrigger => Some(Key::Alt),
        Button::LeftTrigger => Some(Key::Control),
        Button::LeftThumb => Some(Key::Tab),
        Button::RightThumb => Some(Key::Return),
        Button::West => Some(Key::Space),

        // conditional
        Button::RightTrigger2 => {
            if keyboard_mode {
                None
            } else {
                Some(Key::Meta)
            }
        }
        Button::East => {
            if keyboard_mode {
                Some(Key::Backspace)
            } else {
                Some(Key::Escape)
            }
        }

        _ => None,
    }
}
