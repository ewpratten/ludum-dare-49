use std::ops::{Deref, DerefMut};

use raylib::prelude::*;
use raylib::{math::Vector2, prelude::RaylibDraw, RaylibHandle};

#[derive(Debug)]
pub struct HackedRaylibHandle(RaylibHandle);

impl HackedRaylibHandle {
    /// Get the screen size as a vector
    #[inline]
    pub fn get_screen_size(&self) -> Vector2 {
        Vector2::new(
            self.get_screen_width() as f32,
            self.get_screen_height() as f32,
        )
    }

    #[inline]
    pub fn draw_rgb_split_text(
        &mut self,
        position: Vector2,
        text: &str,
        font_size: i32,
        hovering: bool,
        color: Color,
    ) {
        let extra_smudge = if hovering { 2 } else { 0 };
        self.draw_text(
            text,
            position.x as i32 - 1 - extra_smudge,
            position.y as i32,
            font_size,
            Color::BLUE,
        );
        self.draw_text(
            text,
            position.x as i32 + 1 + extra_smudge,
            position.y as i32,
            font_size,
            Color::RED,
        );
        self.draw_text(text, position.x as i32, position.y as i32, font_size, color);
    }
}

impl RaylibDraw for HackedRaylibHandle {}

impl From<RaylibHandle> for HackedRaylibHandle {
    fn from(handle: RaylibHandle) -> Self {
        Self(handle)
    }
}

impl Deref for HackedRaylibHandle {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HackedRaylibHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
