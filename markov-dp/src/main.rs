mod core;
mod graphics;
mod map;
mod mdp;
mod robot;

use crate::core::Core;
use raylib::prelude::*;

pub const N_ROWS: usize = 6;
pub const N_COLS: usize = 8;
pub const N_STATES: usize = N_ROWS * N_COLS;
pub const PROBABILITIES: [f32; 3] = [0.8, 0.1, 0.1];
pub const TILE_SIZE: f32 = 100.0;

pub const DISCOUNT_FACTORS: [f32; 4] = [0.86, 0.90, 0.94, 0.98];
pub const SUCCESS_PROBABILITIES: [f32; 4] = [0.5, 0.7, 0.8, 0.9];

fn main() {
    println!("Iniciando simulaciones paralelas...");

    let results = Core::run_parallel_simulation();

    graphics::graphic(&results);

    println!("Resultados de las simulaciones paralelas:");

    let mut visual_core = Core::new(0, 3);

    let (mut rlib, thread) = raylib::init()
        .size(800, 600)
        .title("MDP Robotics - INFO1167")
        .msaa_4x()
        .vsync()
        .build();

    rlib.set_target_fps(60);

    let camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: Vector2::new(0.001, 0.001),
        rotation: 0.0,
        zoom: 1.0,
    };

    while !rlib.window_should_close() {
        let mut drawer = rlib.begin_drawing(&thread);
        drawer.clear_background(Color::DARKGRAY);

        let mut drawer2d = drawer.begin_mode2D(camera);
        visual_core.simulate(Some(&mut drawer2d));
    }
}
