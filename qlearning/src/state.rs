use crate::{N_COLS, TILE_SIZE};
use raylib::prelude::*;
use utils::Raylib;

#[derive(Debug, Clone)]
pub struct State {
    pub key: &'static str,
    pub r#type: StatusType,
    pub reward: f32,
    pub position: Vector2,
    pub index: usize,
    pub color: Color,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatusType {
    Normal,
    Wall,
    Goal,
}

impl State {
    pub fn new(key: &'static str, i: usize, j: usize) -> State {
        let (r#type, reward, color) = match key.chars().next().unwrap() {
            'G' => (StatusType::Goal, 1.0, Color::GREEN),
            'W' => (StatusType::Wall, -0.1, Color::BLACK),
            'S' => (StatusType::Normal, -0.1, Color::WHITESMOKE),
            _ => panic!("Invalid state key: {}", key),
        };

        let position = Vector2 {
            x: j as f32 * TILE_SIZE,
            y: i as f32 * TILE_SIZE,
        };

        let norm_x = position.x as usize / TILE_SIZE as usize;
        let norm_y = position.y as usize / TILE_SIZE as usize;

        let index = norm_y * N_COLS + norm_x;

        State {
            key,
            r#type,
            reward,
            color,
            position,
            index,
        }
    }

    pub fn draw(&self) {
        Raylib::draw_tile(self.key, self.position, self.color);
    }
}
