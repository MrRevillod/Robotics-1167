mod map;
mod mdp;
mod robot;
mod status;
mod utils;

use crate::map::Map;
use raylib::prelude::*;

// la politica me dice que accion tomar en cada estado
// para eso es el algoritmo de valor iterativo -> politica optima

// lanzar random y armar rangos desde 0 a 1 (0.8, 0.1, 0.1)

fn main() {
    let (mut rlib, thread) = raylib::init()
        .size(800, 600)
        .title("MDP Robotics - INFO1167")
        .msaa_4x()
        .vsync()
        .build();

    rlib.disable_cursor();
    rlib.set_target_fps(60);

    let mut map = Map::new();

    let camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: Vector2::new(0.001, 0.001),
        rotation: 0.0,
        zoom: 1.0,
    };

    while !rlib.window_should_close() {
        let mut drawer = rlib.begin_drawing(&thread);
        drawer.clear_background(Color::DARKGRAY);

        let mut mode2d = drawer.begin_mode2D(camera);
        map.draw(&mut mode2d);
    }
}
