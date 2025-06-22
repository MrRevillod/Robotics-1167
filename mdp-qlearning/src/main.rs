mod environment;
mod state;

use raylib::prelude::*;

use crate::environment::Environment;

pub const N_ROWS: usize = 12;
pub const N_COLS: usize = 15;
pub const N_STATES: usize = 146;
pub const TILE_SIZE: f32 = 75.0;
pub const WINDOW_SIZE: (usize, usize) = (N_COLS * TILE_SIZE as usize, N_ROWS * TILE_SIZE as usize);

pub const LEARNING_RATE: f32 = 0.1;
pub const EPSILON_DECAY: f32 = 0.9;

pub const DISCOUNT_FACTOR: f32 = 0.95;
pub const SUCCESS_PROBABILITIES: [f32; 3] = [0.3, 0.7, 0.9];

pub const GOAL_POSITION: [usize; 2] = [5, 8];

pub const EPISODES: usize = 1000;
pub const MAX_STEPS: usize = 100;

#[rustfmt::skip]
pub const RAW_MAP: [[&str; 15]; 12] = [
    [ "S0",  "S1",  "S2",  "S3",  "S4",  "S5",  "S6",  "S7",  "S8",  "S9",  "S10", "S11", "S12", "S13", "S14" ],
    [ "S15", "S16", "S17", "S18", "S19", "S20", "S21", "S22", "S23", "S24", "S25", "S26", "S27", "S28", "S29" ],
    [ "S30", "S31", "W0",  "W1",  "W2",  "S32", "W3",  "W4",  "W5",  "S33", "S34", "S35", "S36", "S37", "S38" ],
    [ "S39", "S40", "W6",  "S41", "S42", "S43", "S44", "S45", "W7",  "S46", "S47", "S48", "W8",  "W9",  "S49" ],
    [ "S50", "S51", "W10", "S52", "S53", "S54", "S55", "S56", "S57", "S58", "S59", "W11", "W12", "W13", "S60" ],
    [ "S61", "S62", "S63", "S64", "S65", "S66", "S67", "S68", "G",   "S69", "S70", "S71", "S72", "S73", "S74" ],
    [ "S75", "S76", "S77", "S78", "S79", "S80", "S81", "S82", "W14", "W15", "S83", "S84", "S85", "S86", "S87" ],
    [ "S88", "S89", "W16", "W17", "W18", "S90", "S91", "S92", "S93", "W19", "W20", "W21", "S94", "S95", "W22" ],
    [ "S96", "S97", "W23", "S98", "S99", "S100","S101","S102","S103","S104","S105","S106","S107","S108","S109" ],
    [ "S110","S111","W24", "S112","S113","S114","W25", "S115","S116","W26", "W27", "W28", "W29", "S117","S118"],
    [ "S119","S120","S121","S122","S123","S124","S125","S126","S127","S128","S129","S130","S131","S132","S133"],
    [ "S134","S135","S136","S137","W30", "W31", "W32", "S138","S139","S140","S141","S142","S143","S144","W33" ],
];

fn main() {
    let (mut rlib, thread) = raylib::init()
        .size(WINDOW_SIZE.0 as i32, WINDOW_SIZE.1 as i32)
        .title("MDP Robotics - INFO1167")
        .msaa_4x()
        .vsync()
        .log_level(TraceLogLevel::LOG_NONE)
        .build();

    rlib.set_target_fps(60);

    let camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: Vector2::new(0.001, 0.001),
        rotation: 0.0,
        zoom: 1.0,
    };

    let environment = Environment::new(SUCCESS_PROBABILITIES[0]);

    while !rlib.window_should_close() {
        let mut drawer = rlib.begin_drawing(&thread);
        drawer.clear_background(Color::DARKGRAY);
        #[allow(unused_must_use)]
        drawer.begin_mode2D(camera);

        environment.draw();
    }
}
