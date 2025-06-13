use crate::{
    mdp::Mdp,
    robot::Robot,
    status::{Status, StatusType},
    utils::constants::*,
    utils::draw_status_tile,
};

use raylib::prelude::*;

#[derive(Debug)]
pub struct Map {
    robot: Robot,
    map: Vec<Vec<Status>>,
    mdp: Mdp,
}

impl Map {
    pub fn new() -> Self {
        let raw_map = vec![
            ["S0", "S1", "P1", "O1", "S3", "O2", "S4", "S5"],
            ["O3", "S6", "S7", "S8", "S9", "S10", "S11", "O4"],
            ["S12", "P2", "S14", "O5", "S15", "P3", "S17", "S18"],
            ["S19", "S20", "S21", "S22", "M", "S24", "S25", "O6"],
            ["S26", "O7", "O8", "S27", "S28", "S29", "P4", "S31"],
            ["S32", "O9", "S33", "S34", "O10", "S35", "S36", "S37"],
        ];

        let robot = Robot::new(Vector2::new(50.0, 50.0));

        let map: Vec<Vec<Status>> = raw_map
            .into_iter()
            .map(|row| row.iter().map(|&key| Status::from(key)).collect())
            .collect();

        Self {
            robot,
            mdp: Mdp::new(map.clone()),
            map,
        }
    }

    pub fn draw(&mut self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, status) in row.iter().enumerate() {
                let color = match status.r#type {
                    StatusType::Normal => Color::WHITESMOKE,
                    StatusType::Danger => Color::RED,
                    StatusType::Wall => Color::BLACK,
                    StatusType::Goal => Color::GREEN,
                };

                let x = (j as f32 * TILE_SIZE as f32) as i32;
                let y = (i as f32 * TILE_SIZE as f32) as i32;

                draw_status_tile(drawer, status.key, color, (x, y));
            }
        }

        self.robot.draw(drawer);
    }

    pub fn run_value_iteration(&mut self, discount_factor: f32) {
        self.mdp.run_value_iteration(discount_factor);
    }
}
