use crate::{
    TILE_SIZE,
    robot::Robot,
    status::{Status, StatusType},
    utils::draw_status_tile,
};

use raylib::prelude::*;

pub const N_ROWS: usize = 6;
pub const N_COLS: usize = 8;
pub const N_STATES: usize = N_ROWS * N_COLS;

const PROBABILITIES: [f32; 3] = [0.8, 0.1, 0.1];

#[derive(Debug)]
pub struct Map {
    robot: Robot,
    map: Vec<Vec<Status>>,
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

        Self { robot, map }
    }

    pub fn gen_transicion_matrix(&self) -> Vec<Vec<Vec<f32>>> {
        // matrices[action][from][to]
        let mut matrices = vec![
            vec![vec![0.0; N_STATES]; N_STATES], // North
            vec![vec![0.0; N_STATES]; N_STATES], // South
            vec![vec![0.0; N_STATES]; N_STATES], // East
            vec![vec![0.0; N_STATES]; N_STATES], // West
        ];

        // Direcciones: (di, dj)
        let directions = [
            [(-1, 0), (0, -1), (0, 1)], // North: principal, izquierda(W), derecha(E)
            [(1, 0), (0, 1), (0, -1)],  // South: principal, izquierda(E), derecha(W)
            [(0, 1), (-1, 0), (1, 0)],  // East: principal, izquierda(N), derecha(S)
            [(0, -1), (1, 0), (-1, 0)], // West: principal, izquierda(S), derecha(N)
        ];

        for (i, row) in self.map.iter().enumerate() {
            for (j, status) in row.iter().enumerate() {
                let idx = i * N_COLS + j;

                if status.r#type == StatusType::Wall {
                    continue;
                }

                for (action, dirs) in directions.iter().enumerate() {
                    let mut stay_prob = 0.0;

                    for (k, (di, dj)) in dirs.iter().enumerate() {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        if ni >= 0 && ni < N_ROWS as isize && nj >= 0 && nj < N_COLS as isize {
                            let ni = ni as usize;
                            let nj = nj as usize;

                            let next_status = &self.map[ni][nj];
                            let next_idx = ni * N_COLS + nj;

                            if next_status.r#type == StatusType::Wall {
                                stay_prob += PROBABILITIES[k];
                            } else {
                                matrices[action][idx][next_idx] += PROBABILITIES[k];
                            }
                        } else {
                            stay_prob += PROBABILITIES[k];
                        }
                    }

                    matrices[action][idx][idx] += stay_prob;
                }
            }
        }

        matrices
    }

    pub fn gen_reward_vector(&self) -> Vec<f32> {
        self.map
            .iter()
            .flat_map(|row| row.iter().map(|status| status.reward))
            .collect()
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
}
