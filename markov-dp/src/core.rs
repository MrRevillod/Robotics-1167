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
    pub fn new(discount_factor_id: usize) -> Self {
        let map = Map::new();
        let mut mdp = Mdp::new(map.clone());

        let discount_factors = DISCOUNT_FACTORS.to_vec();

        mdp.value_iteration(discount_factors[discount_factor_id]);

        let mut random = StdRng::from_os_rng();

        let initial_position = map.get_random_valid_position(&mut random);
        let robot = Robot::new(initial_position);

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

    pub fn run_parallel_simulation() -> Vec<Vec<f32>> {
        let mut handles = vec![];

        for i in 0..4 {
            let handle = thread::spawn(move || {
                let mut core = Core::new(i);

                while core.simulation_steps < 1000 {
                    core.simulate(None);
                }

                core.rewards
            });

            handles.push(handle);
        }

        let mut results = vec![];

        for handle in handles.into_iter() {
            let res = handle.join().unwrap();
            results.push(res);
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
