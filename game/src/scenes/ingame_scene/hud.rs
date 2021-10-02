use crate::{GameConfig, utilities::render_layer::ScreenSpaceRender};
use raylib::prelude::*;
use super::InGameScreen;

impl ScreenSpaceRender for InGameScreen {
    fn render_screen_space(
        &self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig
    ) {
        // Calculate the logo position
        let screen_size = raylib.get_screen_size();
    }
}
