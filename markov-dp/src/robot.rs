use raylib::prelude::*;

use crate::utils::constants::TILE_SIZE;

#[derive(Debug)]
pub struct Robot {
    position: Vector2,
    color: Color,
}

impl Robot {
    pub fn new(start_position: Vector2) -> Self {
        Self {
            position: start_position,
            color: Color::from_hex("3c40a3").unwrap(),
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_normalized_position(&self) -> usize {
        let pos = self.get_position();

        let norm_x = pos.x as usize / TILE_SIZE as usize;
        let norm_y = pos.y as usize / TILE_SIZE as usize;

        norm_x * norm_y
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    pub fn update(&mut self, policy: Vec<usize>) {
        let next_action = policy[self.get_normalized_position()];

        let diff = match next_action {
            0 => (0, -1),
            1 => (0, 1),
            2 => (1, 0),
            3 => (-1, 0),
            _ => panic!("Invalid direction index"),
        };

        let current_pos = self.get_position();

        self.set_position(Vector2 {
            x: current_pos.x + diff.0 as f32,
            y: current_pos.y + diff.1 as f32,
        });
    }

    pub fn draw(&mut self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        drawer.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            25.0,
            self.color,
        );
    }
}
