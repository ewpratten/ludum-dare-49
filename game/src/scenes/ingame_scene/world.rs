use std::ops::Mul;

use super::InGameScreen;
use crate::{
    character::render::render_character_in_camera_space,
    utilities::{non_ref_raylib::HackedRaylibHandle, render_layer::WorldSpaceRender},
    GameConfig,
};
use raylib::prelude::*;

pub const WORLD_LEVEL_X_OFFSET: f32 = 200.0;

impl WorldSpaceRender for InGameScreen {
    fn render_world_space(
        &self,
        raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>,
        config: &GameConfig,
    ) {
        puffin::profile_function!();

        // Get the current level
        let cur_level = self.levels.get(self.current_level_idx).unwrap();

        // Render the world background
        // self.world_background.render(raylib, Vector2::new(0.0, -1080.0), &self.camera);

        // Render the platform layer
        raylib.draw_texture_v(&cur_level.platform_tex, Vector2::new(WORLD_LEVEL_X_OFFSET, -cur_level.platform_tex.height as f32), Color::WHITE);

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


        // Render the player
        render_character_in_camera_space(raylib, &self.player, &config);
    }
}
