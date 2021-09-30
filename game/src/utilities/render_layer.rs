use raylib::{RaylibHandle, prelude::{RaylibDrawHandle, RaylibMode2D}};

use crate::utilities::non_ref_raylib::HackedRaylibHandle;


pub trait FrameUpdate {
    fn update(&mut self, raylib: &RaylibHandle, delta_seconds: f32);
}

pub trait ScreenSpaceRender {
    fn render_screen_space(&self, raylib: &mut HackedRaylibHandle);
}

pub trait WorldSpaceRender {
    fn render_world_space(&self, raylib: &mut RaylibMode2D<HackedRaylibHandle>);
}
