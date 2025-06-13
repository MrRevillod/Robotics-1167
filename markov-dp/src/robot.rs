use raylib::prelude::*;

#[derive(Debug)]
pub struct Robot {
    position: Vector2,
    color: Color,
}

impl Robot {
    pub fn new(start_position: Vector2) -> Self {
        Self {
            position: start_position,
            color: Color::from_hex("3c40a3").unwrap(),
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    pub fn draw(&mut self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        drawer.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            25.0,
            self.color,
        );
    }
}
