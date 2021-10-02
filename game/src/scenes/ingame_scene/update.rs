use std::ops::Div;

use super::InGameScreen;
use crate::{
    utilities::{non_ref_raylib::HackedRaylibHandle, render_layer::FrameUpdate},
    GameConfig,
};
use chrono::Duration;
use raylib::prelude::*;

impl FrameUpdate for InGameScreen {
    fn update(
        &mut self,
        raylib: &HackedRaylibHandle,
        delta_seconds: &Duration,
        config: &GameConfig,
    ) {
        // Set the camera's offset based on screen size
        self.camera.offset = raylib.get_screen_size().div(Vector2::new(2.0, 1.05));
        self.camera.target = Vector2::new(self.player.position.x, self.camera.target.y);

        // Check the only possible keyboard inputs
        let is_jump = raylib.is_key_pressed(KeyboardKey::KEY_SPACE);
        let is_dash = raylib.is_key_down(KeyboardKey::KEY_LEFT_SHIFT);
        let is_pause = raylib.is_key_down(KeyboardKey::KEY_ESCAPE);

        if is_jump {
            self.player.apply_force(Vector2::new(0.0, -30.0));
        }
        self.player.update_gravity();
    }
}
