use raylib::{math::Rectangle, texture::Texture2D};

use crate::utilities::world_paint_texture::WorldPaintTexture;

pub mod loader;

#[derive(Debug, Deserialize)]
pub struct LevelZones {
    pub appear: Vec<Rectangle>,
    pub disappear: Vec<Rectangle>,
    pub win: Rectangle,
}

#[derive(Debug)]
pub struct Level {
    pub name: String,
    pub background_tex: WorldPaintTexture,
    pub platform_tex: Texture2D,
    pub appearing_platform_tex: Texture2D,
    pub disappearing_platform_tex: Texture2D,
    pub colliders: Vec<Rectangle>,
    pub zones: LevelZones,
}
