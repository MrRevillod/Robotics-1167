use std::{thread, time::Duration};

use raylib::prelude::*;

use crate::{DISCOUNT_FACTORS, map::Map, mdp::Mdp, robot::Robot};

pub struct Core {
    pub map: Map,
    pub mdp: Mdp,
    pub robot: Robot,
    pub simulation_steps: u32,
    pub rewards: Vec<f32>,
}

impl Core {
    pub fn new(discount_factor_id: usize, success_prob: usize) -> Self {
        let map = Map::new();
        let mut mdp = Mdp::new(map.clone());

        let discount_factors = DISCOUNT_FACTORS.to_vec();

        mdp.value_iteration(discount_factors[discount_factor_id]);

        let initial_position = map.get_random_valid_position();
        let robot = Robot::new(initial_position, success_prob);

        Self {
            map,
            mdp,
            robot,
            simulation_steps: 0,
            rewards: vec![],
        }
    }

    pub fn reset_robot(&mut self) {
        let new_position = self.map.get_random_valid_position();
        self.robot.set_position(new_position);
    }

    pub fn run_simulation() -> Vec<Vec<Vec<f32>>> {
        let map = Map::new();
        let transition_matrix = Mdp::build_transition_matrix_static(&map);

        let mut results = vec![vec![vec![]; 4]; 4];
        let discount_factors = DISCOUNT_FACTORS.to_vec();

        let mut mdps = Vec::new();
        for &discount_factor in &discount_factors {
            let mut mdp = Mdp::new_with_transition_matrix(map.clone(), transition_matrix.clone());
            mdp.value_iteration(discount_factor);
            mdps.push(mdp);
        }

        for success_prob in 0..4 {
            for discount_factor in 0..4 {
                let initial_position = map.get_random_valid_position();
                let mut robot = Robot::new(initial_position, success_prob);

                let mut simulation_steps = 0;
                let mut rewards = vec![];

                while simulation_steps < 1000 {
                    robot.update(&mdps[discount_factor].get_max_policy(), &map);

                    simulation_steps += 1;
                    let robot_pos = robot.get_matricial_position();

                    rewards.push(map.states[robot_pos[0]][robot_pos[1]].reward);

                    if robot.get_position() == map.get_goal_position() {
                        let new_position = map.get_random_valid_position();
                        robot.set_position(new_position);
                    }
                }

                results[success_prob][discount_factor] = rewards;
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
