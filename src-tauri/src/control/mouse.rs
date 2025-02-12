use enigo::{Coordinate, Enigo, Mouse};

fn calculate_mouse_movement_value(axis_val: f32) -> i32 {
    if axis_val == 0.0 {
        return 0;
    };
    let rounded_val = axis_val.abs().exp() as i32;
    let movement = rounded_val.pow(2);
    if axis_val.abs() < 0.99 {
        movement
    } else {
        movement * 8
    }
}

pub fn handle_mouse_move(axis_data: (f32, f32), enigo: &mut Enigo) {
    let (x_axis_val, y_axis_val) = axis_data;
    let x_direction = if x_axis_val > 0.0 { 1 } else { -1 };
    let y_direction = if y_axis_val > 0.0 { -1 } else { 1 };
    let x = calculate_mouse_movement_value(x_axis_val);
    let y = calculate_mouse_movement_value(y_axis_val);

    if x == 0 && y == 0 {
        return;
    }

    let _ = enigo.move_mouse(x * x_direction, y * y_direction, Coordinate::Rel);
}

pub fn handle_scroll(axis_data: (f32, f32), enigo: &mut Enigo) {
    let (x_axis_val, y_axis_val) = axis_data;
    let threshold = 0.2;
    let scroll_axis = if x_axis_val.abs() > y_axis_val.abs() {
        enigo::Axis::Horizontal
    } else {
        enigo::Axis::Vertical
    };

    let scroll_axis_val = if scroll_axis == enigo::Axis::Horizontal {
        x_axis_val
    } else {
        y_axis_val
    };

    let scroll = if scroll_axis_val.abs() > threshold {
        scroll_axis_val.signum() as i32
    } else {
        0
    };
    let scroll_direction = if scroll_axis == enigo::Axis::Vertical {
        -1
    } else {
        1
    };
    if scroll != 0 {
        let _ = enigo.scroll(scroll * scroll_direction, scroll_axis);
    }
}
