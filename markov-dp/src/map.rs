use crate::{N_COLS, N_ROWS, TILE_SIZE};
use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct State {
    pub key: &'static str,
    pub r#type: StatusType,
    pub reward: f32,
    pub position: Vector2,
    pub color: Color,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatusType {
    Normal,
    Danger,
    Wall,
    Goal,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub states: Vec<Vec<State>>,
}

impl Map {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let raw_map = vec![
            [ "S0",  "S1",  "P1",  "O1",  "S3",  "O2",  "S4",  "S5"  ],
            [ "O3",  "S6",  "S7",  "S8",  "S9",  "S10", "S11", "O4"  ],
            [ "S12", "P2",  "S14", "O5",  "S15", "P3",  "S17", "S18" ],
            [ "S19", "S20", "S21", "S22", "M",   "S24", "S25", "O6"  ],
            [ "S26", "O7",  "O8",  "S27", "S28", "S29", "P4",  "S31" ],
            [ "S32", "O9",  "S33", "S34", "O10", "S35", "S36", "S37" ],
        ];

        let mut map = Vec::new();

        for (i, row) in raw_map.iter().enumerate() {
            let mut map_row = Vec::new();
            for (j, state_key) in row.iter().enumerate() {
                map_row.push(Self::create_state(state_key, i, j));
            }
            map.push(map_row);
        }

        Self { states: map }
    }

    pub fn draw(&self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        for row in self.states.iter() {
            for state in row.iter() {
                state.draw(drawer);
            }
        }
    }

    fn create_state(key: &'static str, i: usize, j: usize) -> State {
        let (r#type, reward, color) = match key.chars().next().unwrap() {
            'M' => (StatusType::Goal, 10.0, Color::GREEN),
            'P' => (StatusType::Danger, -0.5, Color::RED),
            'O' => (StatusType::Wall, -0.1, Color::BLACK),
            'S' => (StatusType::Normal, -0.1, Color::WHITESMOKE),
            _ => unreachable!(),
        };

        State {
            key,
            r#type,
            reward,
            color,
            position: Vector2 {
                x: j as f32 * TILE_SIZE,
                y: i as f32 * TILE_SIZE,
            },
        }
    }

    pub fn get_goal_position(&self) -> Vector2 {
        Vector2 { x: 450.0, y: 350.0 }
    }

    pub fn is_valid_position(&self, position: Vector2) -> bool {
        let grid_x = (position.x / TILE_SIZE) as usize;
        let grid_y = (position.y / TILE_SIZE) as usize;

        if grid_y >= N_ROWS || grid_x >= N_COLS {
            return false;
        }

        self.states[grid_y][grid_x].r#type != StatusType::Wall
    }

    pub fn get_random_valid_position(&self) -> Vector2 {
        loop {
            let grid_x = rand::random_range(0..N_COLS);
            let grid_y = rand::random_range(0..N_ROWS);

            let position = Vector2::new(
                grid_x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                grid_y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            );

            if self.is_valid_position(position) {
                return position;
            }
        }
    }
}

impl State {
    pub fn draw(&self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        let position = self.position;
        let color = self.color;

        drawer.draw_rectangle(
            position.x as i32,
            position.y as i32,
            TILE_SIZE as i32,
            TILE_SIZE as i32,
            color,
        );

        if self.r#type != StatusType::Wall {
            let text_color = if color == Color::WHITESMOKE {
                Color::BLACK
            } else {
                Color::WHITE
            };

            drawer.draw_text(
                self.key,
                (position.x + TILE_SIZE as f32 / 2.0) as i32 - 12,
                (position.y + TILE_SIZE as f32 / 2.0) as i32 - 12,
                30,
                text_color,
            );
        }

        drawer.draw_rectangle_lines(
            position.x as i32,
            position.y as i32,
            TILE_SIZE as i32,
            TILE_SIZE as i32,
            Color::GRAY,
        );
    }
}
