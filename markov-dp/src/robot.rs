use raylib::prelude::*;

use crate::{N_COLS, N_ROWS, TILE_SIZE, map::Map};

#[derive(Debug)]
pub struct Robot {
    position: Vector2,
}

impl Robot {
    pub fn new(start_position: Vector2) -> Self {
        Self {
            position: start_position,
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_normalized_position(&self) -> usize {
        let pos = self.get_position();

        let norm_x = pos.x as usize / TILE_SIZE as usize;
        let norm_y = pos.y as usize / TILE_SIZE as usize;

        norm_y * N_COLS + norm_x
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    pub fn get_matricial_position(&self) -> [usize; 2] {
        let pos = self.get_position();

        let norm_x = pos.x as usize / TILE_SIZE as usize;
        let norm_y = pos.y as usize / TILE_SIZE as usize;

        [norm_y, norm_x]
    }

    fn calc_next_action(next_action: usize) -> (f32, f32) {
        let north = (0.0, -TILE_SIZE);
        let south = (0.0, TILE_SIZE);
        let east = (TILE_SIZE, 0.0);
        let west = (-TILE_SIZE, 0.0);

        let combinations = [
            [north, east, west],
            [south, east, west],
            [east, north, south],
            [west, south, north],
        ];

        let possible_actions = combinations[next_action];
        let success_prob = 0.8;

        let choice = rand::random::<f32>();

        if choice <= success_prob {
            possible_actions[0] // Acción principal
        } else {
            match rand::random_bool(0.5) {
                true => possible_actions[1],
                false => possible_actions[2],
            }
        }
    }

    pub fn update(&mut self, policy: &Vec<usize>, map: &Map) {
        let current_index = self.get_normalized_position();

        if current_index >= policy.len() {
            return;
        }

        let next_action = policy[current_index];
        let diff = Self::calc_next_action(next_action);

        let current_pos = self.get_position();
        let new_pos = Vector2 {
            x: current_pos.x + diff.0,
            y: current_pos.y + diff.1,
        };

        // Validar que la nueva posición esté dentro de los límites del mapa
        let max_x = (N_COLS as f32) * TILE_SIZE;
        let max_y = (N_ROWS as f32) * TILE_SIZE;

        // Verificar límites del mapa Y que no sea una pared
        if new_pos.x >= 0.0
            && new_pos.x < max_x
            && new_pos.y >= 0.0
            && new_pos.y < max_y
            && map.is_valid_position(new_pos)
        {
            self.set_position(new_pos);
        }
        // Si el movimiento es inválido, el robot se queda en su posición actual
    }

    pub fn draw(&self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        drawer.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            25.0,
            Color::BLUE, // Color fijo por ahora
        );
    }
}
