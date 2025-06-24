use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    DISCOUNT_FACTOR, EPISODES, EPSILON_DECAY, LEARNING_RATE, MAX_STEPS, N_COLS, N_ROWS, N_STATES,
    RAW_MAP, TILE_SIZE,
    state::{State, StatusType},
};

use rand::{random, random_range};
use raylib::prelude::*;
use utils::{Raylib, argmax_f32, num_to_direction};

#[derive(Debug, Clone)]
pub struct Environment {
    pub map: Vec<Vec<State>>,
    pub agent_position: Vector2,
    pub q_table: Vec<Vec<f32>>,
    pub success_prob: f32,
    pub epsilon: f64,
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
            epsilon: 0.1_f64,
        }
    }

    pub fn reset_agent(&mut self) -> State {
        let state = self.get_random_state();
        self.agent_position = state.position;

        state
    }

    pub fn choose_action(&self, state: &State) -> usize {
        if random::<f64>() > self.epsilon {
            return self.get_best_action(state);
        }

        random_range::<usize, _>(0..4)
    }

    pub fn get_best_action(&self, state: &State) -> usize {
        let state_rewards = &self.q_table[state.index];
        argmax_f32(state_rewards)
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
            _ => panic!("Invalid action"),
        };

        if random::<f32>() <= self.success_prob {
            r#move(action);
        }

        if row < 0 || col < 0 || row >= N_ROWS as isize || col >= N_COLS as isize {
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

    /// Actualiza la Q-table usando la ecuación de Bellman para Q-Learning.
    ///
    /// Esta función implementa el núcleo del algoritmo Q-Learning, que permite al robot
    /// aprender la política óptima a través de la experiencia. La actualización sigue
    /// la ecuación: Q(s,a) = Q(s,a) + α[r + γ*max_a'Q(s',a') - Q(s,a)]

    /// LEARNING_RATE: qué tan rápido aprende el robot.
    /// a más alto, más rápido se adapta a nuevas experiencias.
    ///
    /// DISCOUNT_FACTOR: Factor de descuento que determina la importancia
    /// de las recompensas futuras vs inmediatas.
    /// a mayor valor, el robot valora más las recompensas futuras
    ///
    /// r (reward): Recompensa inmediata obtenida al llegar al next_state.
    /// Guía al robot sobre qué tan bueno o malo es estar en ese estado.
    ///
    /// max_future_q: El máximo valor Q esperado desde el siguiente estado,
    /// representando la mejor acción posible que el robot puede tomar desde ahí.
    /// Esto permite que el robot considere las consecuencias futuras de sus acciones.
    pub fn update_q_table(&mut self, current_state: &State, action: usize, next_state: &State) {
        let max_future_q_index = argmax_f32(&self.q_table[next_state.index]);
        let max_future_q = self.q_table[next_state.index][max_future_q_index];
        let old_q = self.q_table[current_state.index][action];

        // Formula: Q_nuevo = (1-α)*Q_viejo + α*(recompensa + γ*mejor_Q_futuro)
        // El balance entre estos términos permite al robot aprender gradualmente
        // sin olvidar completamente lo que ya sabía
        self.q_table[current_state.index][action] = (1.0 - LEARNING_RATE) * old_q
            + LEARNING_RATE * (next_state.reward + DISCOUNT_FACTOR * max_future_q);
    }

    pub fn run(&mut self) -> (Vec<f64>, Vec<usize>) {
        let mut rewards = Vec::new();
        let mut policies = Vec::new();
        let mut steps_per_episode = Vec::new();

        for episode in 0..EPISODES {
            let mut current_state = self.reset_agent();

            let mut steps = 0;
            let mut total_reward = 0.0_f64;

            for _ in 0..MAX_STEPS {
                let action = self.choose_action(&current_state);
                let next_state = self.step(&current_state, action);

                self.update_q_table(&current_state, action, &next_state);

                current_state = next_state.clone();
                total_reward += next_state.reward as f64;
                steps += 1;

                if current_state.r#type == StatusType::Goal {
                    break;
                }
            }

            let policy: Vec<usize> = self
                .q_table
                .iter()
                .map(|actions| argmax_f32(actions))
                .collect();

            policies.push(policy);
            rewards.push(total_reward);
            steps_per_episode.push(steps);

            self.epsilon *= EPSILON_DECAY;

            println!(
                "Episode {}: Steps: {}, Total Reward: {}, Epsilon: {}",
                episode + 1,
                steps,
                total_reward,
                self.epsilon
            );

            self.draw();
        }

        let mut file = BufWriter::new(File::create("results.txt").unwrap());
        for (i, (policy, reward)) in policies.iter().zip(rewards.iter()).enumerate() {
            let policy_str: String = policy
                .iter()
                .map(|a| format!("{}", num_to_direction(*a)))
                .collect::<Vec<_>>()
                .join(",");

            writeln!(
                file,
                "Episode {}: Policy: [{}], Reward: {}",
                i + 1,
                policy_str,
                reward
            )
            .unwrap();
        }

        (rewards, steps_per_episode)
    }

    pub fn get_random_state(&self) -> State {
        loop {
            let i = random_range(0..N_ROWS);
            let j = random_range(0..N_COLS);

            let state = self.map[i][j].clone();

            if state.r#type != StatusType::Wall {
                return state;
            }
        }
    }

    pub fn draw(&self) {
        self.map.iter().for_each(|row| {
            row.iter().for_each(|state| {
                state.draw();
            });
        });

        Raylib::draw_robot(self.agent_position, 20.0);
    }
}
