//! Defines a texture that tiles across the whole screen in world space

use raylib::{
    camera::Camera2D,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibMode2D},
    texture::Texture2D,
    RaylibHandle,
};

use super::non_ref_raylib::HackedRaylibHandle;

#[derive(Debug)]
pub struct WorldPaintTexture {
    texture: Texture2D,
}

impl WorldPaintTexture {
    /// Construct a new world paint texture
    pub fn new(texture: Texture2D) -> Self {
        Self { texture }
    }

    pub fn render(
        &self,
        raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>,
        origin: Vector2,
        camera: &Camera2D,
    ) {
        // Convert the screen edges to world space
        let top_left = raylib.get_screen_to_world2D(Vector2::new(0.0, 0.0), camera);
        let bottom_right = raylib.get_screen_to_world2D(raylib.get_screen_size(), camera);

        // Calculate the distance between the edges and the origin
        let left_edge_distance = top_left.x - origin.x;
        let right_edge_distance = bottom_right.x - origin.x;

        // Calculate the x position to draw the tile in order for there always to be a tile covering the edges
        let left_tile_x =
            (left_edge_distance / self.texture.width as f32).floor() * self.texture.width as f32;
        let right_tile_x =
        left_tile_x + self.texture.width as f32;

        // Render the tiles
        raylib.draw_texture_v(
            &self.texture,
            Vector2::new(left_tile_x, origin.y),
            Color::WHITE,
        );
        raylib.draw_texture_v(
            &self.texture,
            Vector2::new(right_tile_x, origin.y),
            Color::WHITE,
        );
    }
}
