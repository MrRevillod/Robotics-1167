use raylib::prelude::*;

use crate::TILE_SIZE;

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<char>>,
}

// '⚪' -> S
// '🔴' -> Peligro
// '⚫' -> Muro
// '🟢' -> Meta

impl Map {
    pub fn new() -> Self {
        let map = vec![
            vec!['⚪', '⚪', '🔴', '⚫', '⚪', '⚫', '⚪', '⚪' ], 
            vec!['⚫', '⚪', '⚪', '⚪', '⚪', '⚪', '⚪', '⚫' ], 
            vec!['⚪', '🔴', '⚪', '⚫', '⚪', '🔴', '⚪', '⚪' ], 
            vec!['⚪', '⚪', '⚪', '⚪', '🟢', '⚪', '⚪', '⚫' ], 
            vec!['⚪', '⚫', '⚫', '⚪', '⚪', '⚪', '🔴', '⚪' ], 
            vec!['⚪', '⚫', '⚪', '⚪', '⚫', '⚪', '⚪', '⚪' ], 
        ];

        Self { map }
    }

    pub fn draw(&self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {

        for (i, row) in self.map.iter().enumerate() {
            for (j, circle) in row.iter().enumerate() {
                let (color, text) = match circle {
                    '⚪' => (Color::WHITESMOKE, ""),
                    '🔴' => (Color::RED, "P"),
                    '⚫' => (Color::BLACK, ""),
                    '🟢' => (Color::GREEN, "M"),
                    _ => panic!("Unknown circle type: {}", circle),
                };

                let x = j as f32 * TILE_SIZE;
                let y = i as f32 * TILE_SIZE;

                drawer.draw_rectangle(
                    x as i32,
                    y as i32,
                    TILE_SIZE as i32,
                    TILE_SIZE as i32,
                    color
                );

                if !text.is_empty() {
                    drawer.draw_text(
                        text,
                        (x + TILE_SIZE / 2.0) as i32 - 12,
                        (y + TILE_SIZE / 2.0) as i32 - 12,
                        30,
                        Color::WHITE
                    );
                }

                drawer.draw_rectangle_lines(
                    x as i32,
                    y as i32,
                    TILE_SIZE as i32,
                    TILE_SIZE as i32,
                    Color::GRAY
                );
            }
        }
    }
}