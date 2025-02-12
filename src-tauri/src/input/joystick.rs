use gilrs::{Axis, Gamepad};

fn get_gamepad_axis_data(gamepad: Gamepad, axis: Axis) -> f32 {
    return gamepad.axis_data(axis).map_or(0.0, |data| data.value());
}

pub fn get_all_axis_data(gamepad: Gamepad) -> ((f32, f32), (f32, f32)) {
    let left_x = get_gamepad_axis_data(gamepad, Axis::LeftStickX);
    let left_y = get_gamepad_axis_data(gamepad, Axis::LeftStickY);
    let right_x = get_gamepad_axis_data(gamepad, Axis::RightStickX);
    let right_y = get_gamepad_axis_data(gamepad, Axis::RightStickY);
    ((left_x, left_y), (right_x, right_y))
}

pub fn get_index_data(axis_position: (i32, i32)) -> i32 {
    match axis_position {
        (1, 0) => 0,
        (1, 1) => 1,
        (0, 1) => 2,
        (-1, 1) => 3,
        (-1, 0) => 4,
        (-1, -1) => 5,
        (0, -1) => 6,
        (1, -1) => 7,
        _ => -1,
    }
}

pub fn get_filtered_axis_data(axis_position: (f32, f32)) -> (i32, i32) {
    let threshold: f32 = 0.5;
    let x: i32 = if axis_position.0.abs() >= threshold {
        axis_position.0.signum() as i32
    } else {
        0
    };
    let y: i32 = if axis_position.1.abs() >= threshold {
        axis_position.1.signum() as i32
    } else {
        0
    };
    (x, y)
}

pub fn get_keyboard_focus_input(
    left_axis_data: (f32, f32),
    right_axis_data: (f32, f32),
) -> (i32, i32) {
    let filtered_left_axis_data = get_filtered_axis_data(left_axis_data);
    let filtered_right_axis_data = get_filtered_axis_data(right_axis_data);
    if filtered_left_axis_data == (0, 0) && filtered_right_axis_data == (0, 0) {
        return (-1, -1);
    }
    (
        get_index_data(filtered_left_axis_data),
        get_index_data(filtered_right_axis_data),
    )
}
