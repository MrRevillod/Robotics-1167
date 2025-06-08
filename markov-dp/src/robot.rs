
use raylib::prelude::*;
use crate::{map::Tile, TILE_SIZE};

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug)]
pub struct Robot {
    position: Vector2,
    reward: f32,
    color: Color,
    velocity: f32
}

impl Robot {
    pub fn new() -> Self {
        Self { 
            position: Vector2 { 
                x: TILE_SIZE / 2.0, 
                y: TILE_SIZE / 2.0
            }, 
            reward: 0.0,
            color: Color::from_hex("3c40a3").unwrap(),
            velocity: 25.0
        }
    }

    fn reward(&mut self, current_target: Tile) {
        let bonus = match current_target {
            Tile::Goal => 10.0,
            Tile::Danger => -0.5,
            _ => 0.1
        };

        self.reward += bonus;
    }

    pub fn draw(&mut self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        drawer.draw_circle(
            self.position.x as i32, 
            self.position.y as i32, 
            25.0, 
            self.color
        );
    }
}