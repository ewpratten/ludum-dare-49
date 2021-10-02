use crate::GameConfig;
use crate::utilities::render_layer::ScreenSpaceRender;
use crate::utilities::datastore::*;
use raylib::prelude::*;
use super::InGameScreen;


impl ScreenSpaceRender for InGameScreen {
    fn render_screen_space(
        &self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig,
    ) {

        raylib.draw_texture(&self.levels[0].level_texture, 0, 0, Color::WHITE);

    }
}




