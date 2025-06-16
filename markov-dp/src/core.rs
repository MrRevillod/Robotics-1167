use std::{thread, time::Duration};

use rand::{SeedableRng, rngs::StdRng};
use raylib::prelude::*;

use crate::{DISCOUNT_FACTORS, map::Map, mdp::Mdp, robot::Robot};

pub struct Core {
    pub map: Map,
    pub mdp: Mdp,
    pub robot: Robot,
    pub simulation_steps: u32,
    pub rewards: Vec<f32>,
    random: StdRng,
}

impl Core {
    pub fn new(discount_factor_id: usize, success_prob: usize) -> Self {
        let map = Map::new();
        let mut mdp = Mdp::new(map.clone());

        let discount_factors = DISCOUNT_FACTORS.to_vec();

        mdp.value_iteration(discount_factors[discount_factor_id]);

        let mut random = StdRng::from_os_rng();

        let initial_position = map.get_random_valid_position(&mut random);
        let robot = Robot::new(initial_position, success_prob);

        Self {
            map,
            mdp,
            robot,
            simulation_steps: 0,
            rewards: vec![],
            random,
        }
    }

    pub fn reset_robot(&mut self) {
        let new_position = self.map.get_random_valid_position(&mut self.random);
        self.robot.set_position(new_position);
    }

    pub fn run_parallel_simulation() -> Vec<Vec<Vec<f32>>> {
        let mut handles = vec![];

        for success_prob in 0..4 {
            for discount_factor in 0..4 {
                let handle = thread::spawn(move || {
                    let mut core = Core::new(discount_factor, success_prob);

                    while core.simulation_steps < 1000 {
                        core.simulate(None);
                    }

                    core.rewards
                });

                handles.push(handle);
            }
        }

        // Recolectar todos los resultados primero
        let all_results: Vec<Vec<f32>> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();

        // Organizamos los resultados en la matriz 3D
        // results[success_prob][discount_factor] = rewards
        let mut results = vec![vec![vec![]; 4]; 4];
        let mut result_index = 0;

        for success_prob in 0..4 {
            for discount_factor in 0..4 {
                results[success_prob][discount_factor] = all_results[result_index].clone();
                result_index += 1;
            }
        }

        results
    }

    fn get_reward(&self) -> f32 {
        let robot_pos = self.robot.get_matricial_position();
        self.map.states[robot_pos[0]][robot_pos[1]].reward
    }

    pub fn simulate(&mut self, drawer: Option<&mut RaylibMode2D<'_, RaylibDrawHandle<'_>>>) {
        self.robot.update(&self.mdp.get_max_policy(), &self.map);

        if drawer.is_some() {
            thread::sleep(Duration::from_millis(500));
            self.draw(drawer.unwrap());
        }

        self.simulation_steps += 1;
        self.rewards.push(self.get_reward());

        if self.robot.get_position() == self.map.get_goal_position() {
            self.reset_robot();
        }
    }

    pub fn draw(&self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        self.map.draw(drawer);
        self.robot.draw(drawer);
    }
}
