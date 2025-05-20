use crate::{G, HALF_STADIUM_LENGTH, HALF_STADIUM_WIDTH};

use raylib::{
    ffi::{DrawSphere, GetFrameTime},
    prelude::*,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Ball {
    pub shot_velocity: f32, // Velocidad Disparo de la Pelota (m/s)
    pub elev_angle: f32,    // Angulo Elevacion de la Pelota antes del Disparo Plano XY Grados
    pub dir_angle: f32,     // Angulo Direccion de la Pelota (Heading) (Plano XZ) en Grados
    pub y_axis_proj: f32,   // Proyección de la pelota en eje Y (Plano XY)
    pub velocity: Vector3,  // Velocidad de la Pelota (x, y, z)
    pub position: Vector3,  // Posicion de la Pelota (x, y, z)
    pub is_shooting: bool,  // Indicador booleano de si el balón está en el aire
}

impl Ball {
    pub fn new(base_position: Vector3) -> Self {
        let velocity = Vector3::zero();
        let position = Vector3 {
            x: base_position.x,
            y: base_position.y + 0.15,
            z: base_position.z,
        };

        Ball {
            shot_velocity: 0.0,
            elev_angle: 0.0,
            dir_angle: 0.0,
            y_axis_proj: 0.0,
            velocity,
            position,
            is_shooting: false,
        }
    }

    pub fn update(&mut self, base_position: Vector3) {
        if !self.is_shooting {
            self.position = Vector3 {
                x: base_position.x,
                y: base_position.y + 0.15,
                z: base_position.z,
            };
            self.velocity = Vector3::default();
            return;
        }

        let time = Ball::get_delta_time();

        self.position.x += self.velocity.x * time;
        self.position.z += self.velocity.z * time;
        self.position.y += self.velocity.y * time - 0.5 * G * time.powi(2);

        self.velocity.y -= G * time;

        let is_outside_stadium = self.position.x > HALF_STADIUM_WIDTH
            || self.position.x < -HALF_STADIUM_WIDTH
            || self.position.z > HALF_STADIUM_LENGTH
            || self.position.z < -HALF_STADIUM_LENGTH
            || self.position.y < 0.0;

        if is_outside_stadium {
            self.is_shooting = false;
            self.position = Vector3 {
                x: base_position.x,
                y: base_position.y + 0.15,
                z: base_position.z,
            };
        }
    }

    pub fn draw(&self) {
        unsafe { DrawSphere(self.position.into(), 0.15, Color::WHITESMOKE.into()) };
    }

    fn get_delta_time() -> f32 {
        unsafe { GetFrameTime() }
    }
}
