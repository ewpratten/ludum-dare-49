use raylib::{math::Rectangle, texture::Texture2D};

pub mod loader;

#[derive(Debug)]
pub struct Level {
    pub name: String,
    pub background_tex: Texture2D,
    pub platform_tex: Texture2D,
    pub colliders: Vec<Rectangle>
}
