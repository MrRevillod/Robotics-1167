mod ball;
mod camera;
mod robot;
mod stadium;
mod utils;

use raylib::prelude::*;
use stadium::Stadium;

pub const MAX_ROBOTS: u8 = 10;
pub const STADIUM_WIDTH: f32 = 10.0;
pub const STADIUM_LENGTH: f32 = 20.0;

pub const HALF_STADIUM_WIDTH: f32 = STADIUM_WIDTH / 2.0;
pub const HALF_STADIUM_LENGTH: f32 = STADIUM_LENGTH / 2.0;

pub const STADIUM_FLOOR_THICKNESS: f32 = 0.1;
pub const ROBOT_RADIUS: f32 = 0.5;

pub const G: f32 = 9.8;

pub const BLUE_RING_POSITION: Vector3 = Vector3 {
    x: 0.0,
    y: 1.25,
    z: HALF_STADIUM_LENGTH - 0.55,
};

pub const RED_RING_POSITION: Vector3 = Vector3 {
    x: 0.0,
    y: 1.25,
    z: -(HALF_STADIUM_LENGTH - 0.55),
};

pub const PI: f32 = std::f32::consts::PI;

fn main() {
    let (mut rlib, thread) = raylib::init()
        .size(950, 800)
        .title("Basketbots - Robotics INFO1167")
        .msaa_4x()
        .vsync()
        .build();

    rlib.disable_cursor();
    rlib.set_target_fps(60);

    let mut camera = camera::init();
    let mut stadium = Stadium::new(&mut rlib, &thread);

    while !rlib.window_should_close() {
        camera::update(&rlib, &mut camera);

        let mut drawer = rlib.begin_drawing(&thread);
        drawer.clear_background(Color::BLACK);

        stadium.update();
        stadium.draw(&mut drawer.begin_mode3D(camera));
    }
}
