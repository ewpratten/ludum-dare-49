use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::RaylibDraw,
    texture::Texture2D,
};

#[derive(Debug)]
pub struct AnimatedSpriteSheet {
    texture: Texture2D,
    sprite_size: Vector2,
    sheet_width: usize,
    pub sprite_count: usize,
    pub default_sprite_id: usize,
}

impl AnimatedSpriteSheet {
    /// Construct a new AnimatedSpriteSheet
    pub fn new(
        texture: Texture2D,
        sprite_size: Vector2,
        sheet_width: usize,
        sprite_count: usize,
        default_sprite_id: usize,
    ) -> Self {
        Self {
            texture,
            sprite_size,
            sheet_width,
            sprite_count,
            default_sprite_id,
        }
    }

    pub fn render<T>(
        &self,
        raylib: &mut T,
        position: Vector2,
        scaled_size: Option<Vector2>,
        sprite_id: Option<usize>,
    ) where
        T: RaylibDraw,
    {
        let sprite_id = sprite_id.unwrap_or(self.default_sprite_id);
        let sprite_id = if sprite_id >= self.sprite_count {
            self.default_sprite_id
        } else {
            sprite_id
        };

        let sprite_rect = Rectangle::new(
            (sprite_id % self.sheet_width) as f32 * self.sprite_size.x,
            (sprite_id / self.sheet_width) as f32 * self.sprite_size.y,
            self.sprite_size.x,
            self.sprite_size.y,
        );

        let scaled_size = scaled_size.unwrap_or(self.sprite_size);

        raylib.draw_texture_pro(
            &self.texture,
            sprite_rect,
            Rectangle::new(position.x, position.y, scaled_size.x, scaled_size.y),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }

    // {
    //     let sprite_id = match sprite_id {
    //         Some(id) => {
    //             if id >= self.sprite_count {
    //                 self.default_sprite_id
    //             } else {
    //                 id
    //             }
    //         }
    //         None => self.default_sprite_id,
    //     };

    //     let sprite_x = sprite_id % self.sheet_width;
    //     let sprite_y = sprite_id / self.sheet_width;

    //     raylib.draw_texture_pro(
    //         &self.texture,
    //         Rectangle {
    //             x: sprite_x as f32,
    //             y: sprite_y as f32,
    //             width: self.sprite_size.x,
    //             height: self.sprite_size.y,
    //         },
    //         Rectangle {
    //             x: position.x,
    //             y: position.y,
    //             width: self.sprite_size.x,
    //             height: self.sprite_size.y,
    //         },
    //         Vector2::zero(),
    //         0.0,
    //         Color::WHITE,
    //     );
    // }
}
