use std::ops::{Deref, DerefMut};

use raylib::{
    texture::{RaylibTexture2D, RenderTexture2D},
    RaylibHandle, RaylibThread,
};

/// A texture that resizes with the screen size
pub struct DynScreenTexture {
    texture: RenderTexture2D,
}

impl DynScreenTexture {
    /// Construct a new dynamic screen texture.
    pub fn new(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, String> {
        Ok(Self {
            texture: raylib.load_render_texture(
                thread,
                raylib.get_screen_width() as u32,
                raylib.get_screen_height() as u32,
            )?,
        })
    }

    /// Handle updating the texture
    pub fn update(
        &mut self,
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<(), String> {
        puffin::profile_function!();
        // Check if the window has been resized
        if self.texture.width() != raylib.get_screen_width()
            || self.texture.height() != raylib.get_screen_height()
        {
            self.texture = raylib.load_render_texture(
                thread,
                raylib.get_screen_width() as u32,
                raylib.get_screen_height() as u32,
            )?;
        }
        Ok(())
    }

}

impl Deref for DynScreenTexture {
    type Target = RenderTexture2D;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl DerefMut for DynScreenTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.texture
    }
}
