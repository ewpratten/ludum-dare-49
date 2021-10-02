use raylib::{prelude::RaylibMode2D, RaylibHandle};

use crate::{GameConfig, context::GameContext, utilities::non_ref_raylib::HackedRaylibHandle};

pub trait FrameUpdate {
    fn update(&mut self, raylib: &HackedRaylibHandle, delta_seconds: &chrono::Duration, config: &GameConfig);
}

pub trait ScreenSpaceRender {
    fn render_screen_space(&mut self, raylib: &mut HackedRaylibHandle, config: &GameConfig);
}

pub trait WorldSpaceRender {
    fn render_world_space(&mut self, raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>, config: &GameConfig);
}
