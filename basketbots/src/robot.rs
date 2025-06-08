use crate::{
    BLUE_RING_POSITION, G, HALF_STADIUM_LENGTH, HALF_STADIUM_WIDTH, RED_RING_POSITION,
    ROBOT_RADIUS, utils::random,
};

use rand::Rng;
use raylib::ffi::GetFrameTime;
use raylib::prelude::*;

use super::ball::Ball;

#[derive(Debug, Clone)]
pub struct Robot {
    pub angle: f32,
    pub velocity: f32,
    pub n_step: i16,
    pub position: Vector3,
    pub color: &'static str,
    pub ball: Ball,
}

impl Robot {
    pub fn new(iter: u8) -> Self {
        let rd_angle = Robot::get_rd_angle();
        let rd_velocity = Robot::get_rd_velocity() * Robot::get_delta_time();

        let position = Vector3 {
            x: random().gen_range(-HALF_STADIUM_WIDTH..=HALF_STADIUM_WIDTH),
            y: 0.05,
            z: random().gen_range(-HALF_STADIUM_LENGTH..=HALF_STADIUM_LENGTH),
        };

        let color = match iter % 2 == 0 {
            true => "red_robot",
            false => "blue_robot",
        };

        Robot {
            angle: rd_angle,
            velocity: rd_velocity,
            n_step: random().gen_range(10..=200),
            position,
            color,
            ball: Ball::new(position),
        }
    }

    pub fn draw(&self, drawer: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        let color = match self.color {
            "red_robot" => Color::DARKRED,
            "blue_robot" => Color::DARKBLUE,
            _ => Color::WHITE,
        };

        drawer.draw_cylinder(
            Vector3::new(self.position.x, self.position.y, self.position.z),
            0.25,
            0.25,
            0.15,
            32,
            Color::BLACK,
        );

        drawer.draw_cylinder(
            Vector3::new(self.position.x, self.position.y, self.position.z),
            0.2,
            0.2,
            0.17,
            32,
            color,
        );

        self.ball.draw();
    }

    pub fn update(&mut self) {
        self.n_step -= 1;

        if self.n_step <= 0 {
            let n_step = random().gen_range(10..=200);

            let rd_angle = Robot::get_rd_angle();
            let rd_velocity = Robot::get_rd_velocity() * Robot::get_delta_time();

            self.angle = rd_angle;
            self.n_step = n_step;
            self.velocity = rd_velocity;
        }

        let position = Vector3 {
            x: self.velocity * self.angle.sin(),
            y: 0.05,
            z: self.velocity * self.angle.cos(),
        };

        self.position.x += position.x;
        self.position.z += position.z;

        if self.position.x > HALF_STADIUM_WIDTH - ROBOT_RADIUS {
            self.position.x = HALF_STADIUM_WIDTH - ROBOT_RADIUS;
        }

        if self.position.x < -HALF_STADIUM_WIDTH + ROBOT_RADIUS {
            self.position.x = -HALF_STADIUM_WIDTH + ROBOT_RADIUS;
        }

        if self.position.z > HALF_STADIUM_LENGTH - ROBOT_RADIUS {
            self.position.z = HALF_STADIUM_LENGTH - ROBOT_RADIUS;
        }

        if self.position.z < -HALF_STADIUM_LENGTH + ROBOT_RADIUS {
            self.position.z = -HALF_STADIUM_LENGTH + ROBOT_RADIUS;
        }

        if random().r#gen::<f32>() < 0.1 && !self.ball.is_shooting {
            self.shot_ball();
        }

        self.ball.update(self.position);
    }

    fn shot_ball(&mut self) {
        self.ball.position = self.position;

        let target = match self.color {
            "red_robot" => BLUE_RING_POSITION,
            "blue_robot" => RED_RING_POSITION,
            _ => panic!("Invalid robot color"),
        };

        let dx = target.x - self.position.x;
        let dz = target.z - self.position.z;
        let dy = target.y - self.position.y;

        let horizontal_dist = (dx * dx + dz * dz).sqrt();
        let dir_angle = dz.atan2(dx);

        let v = Robot::get_rd_shot_velocity();

        let v_squared = v * v;
        let sqrt_term = v_squared * v_squared
            - G * (G * horizontal_dist * horizontal_dist + 2.0 * dy * v_squared);

        if sqrt_term < 0.0 {
            return;
        }

        let sqrt_root = sqrt_term.sqrt();
        let elev_angle = ((v_squared + sqrt_root) / (G * horizontal_dist)).atan();

        self.ball.shot_velocity = v;
        self.ball.elev_angle = elev_angle;
        self.ball.dir_angle = dir_angle;
        self.ball.is_shooting = true;

        let vxz = v * elev_angle.cos();

        self.ball.velocity = Vector3 {
            x: vxz * dir_angle.cos(),
            y: v * elev_angle.sin(),
            z: vxz * dir_angle.sin(),
        };
    }

    fn get_rd_angle() -> f32 {
        let angles = [
            0, 15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210, 225, 240, 255, 270,
            285, 300, 315, 330, 345,
        ];

        let angle = angles[random().gen_range(0..=angles.len() - 1)] as f32;

        angle.to_radians()
    }

    fn get_rd_velocity() -> f32 {
        let velocities = [1.0, 2.0, 4.0, 5.0, 6.0];
        velocities[random().gen_range(0..=velocities.len() - 1)]
    }

    fn get_rd_shot_velocity() -> f32 {
        let velocities = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        velocities[random().gen_range(0..=velocities.len() - 1)] as f32
    }

    fn get_delta_time() -> f32 {
        unsafe { GetFrameTime() }
    }
}
