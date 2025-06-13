use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibMode2D};

pub mod constants {
    pub const N_ROWS: usize = 6;
    pub const N_COLS: usize = 8;
    pub const N_STATES: usize = N_ROWS * N_COLS;
    pub const PROBABILITIES: [f32; 3] = [0.8, 0.1, 0.1];
    pub const TILE_SIZE: f32 = 100.0;
}

use constants::TILE_SIZE;

pub fn draw_status_tile(
    drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>,
    text: &str,
    color: Color,
    position: (i32, i32),
) {
    let (x, y) = position;

    drawer.draw_rectangle(x, y, TILE_SIZE as i32, TILE_SIZE as i32, color);

    if !text.is_empty() {
        let text_color = if color == Color::WHITESMOKE {
            Color::BLACK
        } else {
            Color::WHITE
        };

        drawer.draw_text(
            text,
            (x + TILE_SIZE as i32 / 2) - 12,
            (y + TILE_SIZE as i32 / 2) - 12,
            30,
            text_color,
        );
    }

    drawer.draw_rectangle_lines(x, y, TILE_SIZE as i32, TILE_SIZE as i32, Color::GRAY);
}
