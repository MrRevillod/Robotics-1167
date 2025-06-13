use crate::TILE_SIZE;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibMode2D};

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
