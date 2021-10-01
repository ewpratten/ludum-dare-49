pub mod render;

use raylib::math::Vector2;

#[derive(Debug, Clone)]
pub struct MainCharacter {
    pub position: Vector2,
}

impl MainCharacter {

    pub fn new(position: Vector2) -> Self {
        Self { position }
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.position += force;
    }
}
