use crate::{
    DISCOUNT_FACTOR, EPISODES, EPSILON_DECAY, LEARNING_RATE, MAX_STEPS, N_COLS, N_ROWS, N_STATES,
    RAW_MAP, TILE_SIZE,
    state::{State, StatusType},
};

use raylib::prelude::*;
use utils::{Raylib, argmax_f32};

pub struct Environment {
    pub map: Vec<Vec<State>>,
    pub agent_position: Vector2,
    pub q_table: Vec<Vec<f32>>,
    pub success_prob: f32,
    pub epsilon: f32,
}

impl Environment {
    pub fn new(success_prob: f32) -> Self {
        let mut map = Vec::new();

        for (i, row) in RAW_MAP.iter().enumerate() {
            let mut map_row = Vec::new();

            for (j, state_key) in row.iter().enumerate() {
                map_row.push(State::new(state_key, i, j));
            }

            map.push(map_row);
        }

        Self {
            map,
            agent_position: Vector2::default(),
            q_table: vec![vec![0.0; 4]; N_STATES],
            success_prob,
            epsilon: 0.1,
        }
    }

    pub fn reset_agent(&mut self) -> State {
        let state = self.get_random_state();
        self.agent_position = state.position;

        state
    }

    pub fn choose_action(&self, state: &State) -> usize {
        if rand::random::<f32>() > self.epsilon {
            return self.get_best_action(state);
        }

        rand::random_range::<usize, _>(0..4)
    }

    pub fn get_best_action(&self, state: &State) -> usize {
        let state_rewards = &self.q_table[state.index];
        utils::argmax_f32(state_rewards)
    }

    pub fn get_state(&self, i: usize, j: usize) -> &State {
        &self.map[i][j]
    }

    pub fn step(&mut self, current_state: &State, action: usize) -> State {
        let mut row = (self.agent_position.y / TILE_SIZE) as isize;
        let mut col = (self.agent_position.x / TILE_SIZE) as isize;

        let mut r#move = |action: usize| match action {
            0 => row -= 1,
            1 => row += 1,
            2 => col -= 1,
            3 => col += 1,
            _ => panic!("Invalid action "),
        };

        if rand::random::<f32>() <= self.success_prob {
            r#move(action);
        }

        if row < 0 || col < 0 {
            return current_state.clone();
        }

        if self.get_state(row as usize, col as usize).r#type == StatusType::Wall {
            return current_state.clone();
        }

        self.agent_position = Vector2 {
            x: col as f32 * TILE_SIZE,
            y: row as f32 * TILE_SIZE,
        };

        self.get_state(row as usize, col as usize).clone()
    }

    pub fn update_q_table(&mut self, current_state: &State, action: usize, next_state: &State) {
        let max_future_q_index = argmax_f32(&self.q_table[next_state.index]);
        let max_future_q = self.q_table[next_state.index][max_future_q_index];
        let old_q = self.q_table[current_state.index][action];

        self.q_table[current_state.index][action] = (1.0 - LEARNING_RATE) * old_q
            + LEARNING_RATE * (next_state.reward + DISCOUNT_FACTOR * max_future_q);
    }

    pub fn run(&mut self) {
        for episode in 0..EPISODES {
            let mut current_state = self.reset_agent();

            let mut total_reward = 0.0;
            let mut steps = 0;

            for _ in 0..MAX_STEPS {
                let action = self.choose_action(&current_state);
                let next_state = self.step(&current_state, action);

                self.update_q_table(&current_state, action, &next_state);

                current_state = next_state.clone();
                total_reward += next_state.reward;
                steps += 1;

                if current_state.r#type == StatusType::Goal {
                    break;
                }
            }

            self.epsilon *= EPSILON_DECAY;
        }
    }

    pub fn draw(&self) {
        self.map.iter().for_each(|row| {
            row.iter().for_each(|state| state.draw());
        });

        Raylib::draw_robot(self.agent_position, 20.0);
    }

    pub fn get_random_state(&self) -> State {
        loop {
            let i = rand::random_range(0..N_ROWS);
            let j = rand::random_range(0..N_COLS);

            let state = self.map[i][j].clone();

            if state.r#type != StatusType::Wall {
                return state;
            }
        }
    }
}
