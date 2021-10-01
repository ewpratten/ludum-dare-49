use super::InGameScreen;
use crate::{
    character::render::render_character_in_camera_space,
    utilities::{non_ref_raylib::HackedRaylibHandle, render_layer::WorldSpaceRender},
};
use raylib::prelude::*;

impl WorldSpaceRender for InGameScreen {
    fn render_world_space(&self, raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>) {
        // Render the player
        render_character_in_camera_space(raylib, &self.player);
    }
}
