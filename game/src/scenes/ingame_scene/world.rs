use std::ops::Mul;

use super::InGameScreen;
use crate::{
    character::render::render_character_in_camera_space,
    utilities::{non_ref_raylib::HackedRaylibHandle, render_layer::WorldSpaceRender},
    GameConfig,
};
use raylib::prelude::*;

impl WorldSpaceRender for InGameScreen {
    fn render_world_space(
        &self,
        raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>,
        config: &GameConfig,
    ) {
        // Render the player
        render_character_in_camera_space(raylib, &self.player, &config);

        // Render the floor as a line
        let screen_world_zero = raylib.get_screen_to_world2D(Vector2::zero(), self.camera);
        let screen_world_size =
            raylib.get_screen_to_world2D(raylib.get_screen_size().mul(2.0), self.camera);

        raylib.draw_rectangle(
            screen_world_zero.x as i32,
            0,
            screen_world_size.x as i32,
            5,
            config.colors.white,
        );
    }
}
