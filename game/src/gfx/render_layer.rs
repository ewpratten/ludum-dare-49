use raylib::{RaylibHandle, prelude::{RaylibDrawHandle, RaylibMode2D}};


pub trait FrameUpdate {
    fn update(&mut self, raylib: &RaylibHandle, delta_seconds: f32);
}

pub trait ScreenSpaceRender {
    fn render_screen_space(&self, raylib: &mut RaylibDrawHandle);
}

pub trait WorldSpaceRender {
    fn render_world_space(&self, raylib: &mut RaylibMode2D<RaylibDrawHandle>);
}
