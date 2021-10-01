use raylib::{prelude::RaylibMode2D, RaylibHandle};

use crate::utilities::non_ref_raylib::HackedRaylibHandle;

pub trait FrameUpdate {
    fn update(&mut self, raylib: &HackedRaylibHandle, delta_seconds: &chrono::Duration);
}

pub trait ScreenSpaceRender {
    fn render_screen_space(&self, raylib: &mut HackedRaylibHandle);
}

pub trait WorldSpaceRender {
    fn render_world_space(&self, raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>);
}
