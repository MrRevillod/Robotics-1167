use raylib::prelude::*;
use std::collections::HashMap;

use super::robot::Robot;

use crate::utils;
use crate::{HALF_STADIUM_LENGTH, MAX_ROBOTS, STADIUM_LENGTH, STADIUM_WIDTH};

const LINE_THICKNESS: f32 = 0.1;
const FLOOR_THICKNESS: f32 = 0.1;
const LINE_HEIGHT: f32 = 0.01;

#[derive(Debug)]
pub struct Stadium {
    robots: Vec<Robot>,
    models: HashMap<&'static str, Model>,
}

impl Stadium {
    pub fn new(rlib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut robots = Vec::new();
        let models = utils::load_models(rlib, thread);

        for i in 1..=MAX_ROBOTS {
            robots.push(Robot::new(i));
        }

        Self { models, robots }
    }

    pub fn draw(&mut self, drawer: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        self.draw_floor(drawer);
        self.draw_hoops(drawer);

        for robot in &mut self.robots {
            robot.draw(drawer);
        }
    }

    pub fn update(&mut self) {
        for robot in &mut self.robots {
            robot.update();
        }
    }

    fn draw_floor(&self, drawer: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        // Draw the stadium floor

        drawer.draw_cube(
            Vector3::new(0.0, 0.0, 0.0),
            STADIUM_WIDTH,
            FLOOR_THICKNESS,
            STADIUM_LENGTH,
            Color::DARKSLATEGRAY,
        );

        // Draw stadium floor outside white lines

        drawer.draw_cube(
            Vector3::new(0.0, -0.005, 0.0),
            STADIUM_WIDTH + 0.2,
            FLOOR_THICKNESS,
            STADIUM_LENGTH + 0.2,
            Color::WHITESMOKE,
        );

        // Draw the stadium black platform

        drawer.draw_cube(
            Vector3::new(0.0, -10.0, 0.0),
            STADIUM_WIDTH,
            20.0,
            STADIUM_LENGTH * 2.0,
            Color::BLACK,
        );

        // Draw the center line

        drawer.draw_cube(
            Vector3::new(0.0, 0.08, 0.0),
            STADIUM_WIDTH + 0.1,
            LINE_HEIGHT,
            LINE_THICKNESS,
            Color::WHITESMOKE,
        );

        // Draw the center circle perimeter

        drawer.draw_cylinder(
            Vector3::new(0.0, 0.05, 0.0),
            2.0,         // radio arriba
            2.0,         // radio abajo
            LINE_HEIGHT, // altura
            64,          // cantidad de lados
            Color::WHITESMOKE,
        );

        // Draw the center circle content

        drawer.draw_cylinder(
            Vector3::new(0.0, 0.05, 0.0),
            1.9,                // radio arriba
            1.9,                // radio abajo
            LINE_HEIGHT + 0.01, // altura
            64,                 // cantidad de lados
            Color::DARKSLATEGRAY,
        );
    }

    fn draw_hoops(&self, drawer: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        drawer.draw_model_ex(
            &self.models["blue_hoop"],
            Vector3::new(-0.35, -4.0, HALF_STADIUM_LENGTH + 1.0),
            Vector3::new(0.0, 1.0, 1.0),
            180.0,
            Vector3::new(0.018, 0.018, 0.018),
            Color::WHITESMOKE,
        );

        drawer.draw_model_ex(
            &self.models["red_hoop"],
            Vector3::new(0.35, -4.0, -HALF_STADIUM_LENGTH - 1.0),
            Vector3::new(1.0, 0.0, 0.0),
            270.0,
            Vector3::new(0.018, 0.018, 0.018),
            Color::WHITESMOKE,
        );
    }
}
