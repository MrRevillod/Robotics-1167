use raylib::{color::Color, prelude::*};

const TILE_SIZE: f32 = 75.0;

pub struct Raylib {}

impl Raylib {
    pub fn draw_tile(key: &str, position: Vector2, color: Color) {
        let Vector2 { x, y } = position;
        let tile_size = TILE_SIZE as i32;

        unsafe {
            raylib::ffi::DrawRectangle(x as i32, y as i32, tile_size, tile_size, color.into());
            raylib::ffi::DrawRectangleLines(
                x as i32,
                y as i32,
                tile_size,
                tile_size,
                Color::BLACK.into(),
            );

            let font_size = 16;
            let estimated_text_width = key.len() as f32 * (font_size as f32 * 0.5);
            let text_position = Vector2 {
                x: position.x + (TILE_SIZE / 2.0) - (estimated_text_width / 2.0),
                y: position.y + (TILE_SIZE / 2.0) - (font_size as f32 / 2.0),
            };

            let text_color = if color == Color::WHITESMOKE {
                Color::BLACK
            } else {
                Color::WHITE
            };

            Raylib::draw_text(key, text_position, text_color, font_size);
        }
    }

    pub fn draw_text(text: &str, position: Vector2, color: Color, font_size: i32) {
        use std::ffi::CString;

        let c_text = CString::new(text).unwrap_or_else(|_| CString::new("").unwrap());

        unsafe {
            raylib::ffi::DrawText(
                c_text.as_ptr(),
                position.x as i32,
                position.y as i32,
                font_size,
                color.into(),
            );
        }
    }

    pub fn draw_robot(position: Vector2, radius: f32) {
        unsafe {
            raylib::ffi::DrawCircle(
                position.x as i32,
                position.y as i32,
                radius as f32,
                Color::RED.into(),
            );
        }
    }
}

pub fn argmax_f32(values: &[f32]) -> usize {
    assert!(
        values.len() == 4,
        "El vector debe tener exactamente 4 elementos."
    );

    let mut max_index = 0;
    let mut max_value = values[0];

    for (i, &val) in values.iter().enumerate().skip(1) {
        if val > max_value {
            max_value = val;
            max_index = i;
        }
    }

    max_index
}
