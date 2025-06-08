mod map;
mod robot;

use raylib::prelude::*;

use crate::map::Map;

pub const PI: f32 = std::f32::consts::PI;

pub const TILE_SIZE: f32 = 100.0;

fn main() {
    let (mut rlib, thread) = raylib::init()
        .size(800, 600)
        .title("Basketbots - Robotics INFO1167")
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
        drawer.clear_background(Color::BLACK);

        map.draw(&mut drawer.begin_mode2D(camera));
    }
}
