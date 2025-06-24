mod environment;
mod graphics;
mod state;

use crate::environment::Environment;

pub const N_ROWS: usize = 12;
pub const N_COLS: usize = 15;
pub const N_STATES: usize = 180;
pub const TILE_SIZE: f32 = 75.0;
pub const WINDOW_SIZE: (usize, usize) = (N_COLS * TILE_SIZE as usize, N_ROWS * TILE_SIZE as usize);

pub const LEARNING_RATE: f32 = 0.1;
pub const EPSILON_DECAY: f64 = 0.9;

pub const DISCOUNT_FACTOR: f32 = 0.95;
pub const SUCCESS_PROBABILITIES: [f32; 3] = [0.3, 0.7, 0.9];

pub const GOAL_POSITION: [usize; 2] = [5, 8];

pub const EPISODES: usize = 10000;
pub const MAX_STEPS: usize = 1000;

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

use graphics::*;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibMode2DExt},
};
use utils::Raylib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reward_data = Vec::new();
    let mut step_data = Vec::new();

    let mut graphic_simulation = Environment::new(SUCCESS_PROBABILITIES[0]);
    let mut graphic_q_table = vec![vec![0.0; 4]; N_STATES];

    let (mut rlib, thread) = Raylib::init_window("MDP Q-Learning Simulation", WINDOW_SIZE);

    for &prob in &SUCCESS_PROBABILITIES {
        println!("Running simulation with P = {}", prob);
        let mut env = Environment::new(prob);
        let (rewards, steps) = env.run();

        reward_data.push(rewards);
        step_data.push(steps);

        graphic_simulation = env.clone();
        graphic_q_table = env.q_table.clone();
    }

    plot_rewards_and_steps(&reward_data, &step_data)?;

    graphic_simulation.q_table = graphic_q_table;
    graphic_simulation.reset_agent();

    println!("Starting graphical simulation with optimal policy...");

    let mut episode_active = false;
    let mut current_state = graphic_simulation.reset_agent();
    let mut step_count = 0;
    let mut frame_counter = 0;

    while !rlib.window_should_close() {
        let mut d = rlib.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        let _ = d.begin_mode2D(utils::Raylib::camera_2d());

        graphic_simulation.draw();

        frame_counter += 1;
        if frame_counter >= 30 {
            frame_counter = 0;

            if !episode_active {
                current_state = graphic_simulation.reset_agent();
                step_count = 0;
                episode_active = true;
                println!("Starting new episode with optimal policy");
            }

            if episode_active && step_count < MAX_STEPS {
                let action = graphic_simulation.get_best_action(&current_state);
                let next_state = graphic_simulation.step(&current_state, action);

                current_state = next_state.clone();
                step_count += 1;

                if current_state.r#type == crate::state::StatusType::Goal {
                    println!("Goal reached in {} steps!", step_count);
                    episode_active = false;
                }
            } else if step_count >= MAX_STEPS {
                println!("Episode ended - max steps reached");
                episode_active = false;
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }
    }

    Ok(())
}
