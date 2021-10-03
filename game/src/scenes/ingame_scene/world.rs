use std::ops::{Div, Mul, Sub};

use super::InGameScreen;
use crate::{
    character::render::render_character_in_camera_space,
    utilities::{
        math::interpolate_exp, non_ref_raylib::HackedRaylibHandle, render_layer::WorldSpaceRender,
    },
    GameConfig,
};
use raylib::prelude::*;
use tracing::trace;

pub const WORLD_LEVEL_X_OFFSET: f32 = 200.0;
pub const APPEAR_FADE_DISTANCE: f32 = 16.0;

impl WorldSpaceRender for InGameScreen {
    fn render_world_space(
        &mut self,
        raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>,
        config: &GameConfig,
    ) {
        puffin::profile_function!();

        // Get the current level
        let cur_level = self.levels.get(self.current_level_idx).unwrap();

        // Render the world background
        cur_level
            .background_tex
            .render(raylib, Vector2::new(0.0, -1080.0), &self.camera);

        // Render the platform layer
        raylib.draw_texture_v(
            &cur_level.platform_tex,
            Vector2::new(WORLD_LEVEL_X_OFFSET, -cur_level.platform_tex.height as f32),
            Color::WHITE,
        );

        // Calculate the distance between the player and the nearest appearing zone
        let appear_zone_dist = cur_level
            .zones
            .appear
            .iter()
            .map(|zone| {
                Vector2::new(
                    zone.x + WORLD_LEVEL_X_OFFSET + (zone.width / 2.0),
                    zone.y - cur_level.platform_tex.height as f32,
                )
                .distance_to(self.player.position) as i32
            })
            .min()
            .unwrap();
        let opacity = interpolate_exp(
            (appear_zone_dist as f32).sub(APPEAR_FADE_DISTANCE.div(2.0)).div(APPEAR_FADE_DISTANCE).mul(-1.0),
            -APPEAR_FADE_DISTANCE..APPEAR_FADE_DISTANCE,
            0.0..1.0,
            8.0,
        );
        trace!("Appearing values: ({}, {})", appear_zone_dist, opacity);

        // Render the appearing layer
        raylib.draw_texture_v(
            &cur_level.appearing_platform_tex,
            Vector2::new(
                WORLD_LEVEL_X_OFFSET,
                -cur_level.appearing_platform_tex.height as f32,
            ),
            Color::WHITE.fade(opacity),
        );

        #[cfg(all(debug_assertions, feature = "collider_debug"))]
        {
            for collider in &cur_level.colliders {
                let mut translated_collider = collider.clone();
                translated_collider.y += -cur_level.platform_tex.height as f32;
                translated_collider.x += WORLD_LEVEL_X_OFFSET;
                raylib.draw_rectangle_lines_ex(translated_collider, 5, Color::RED);
            }
        }

        // Render the floor as a line
        let screen_world_zero = raylib.get_screen_to_world2D(Vector2::zero(), self.camera);
        let screen_world_size =
            raylib.get_screen_to_world2D(raylib.get_screen_size().mul(2.0), self.camera);

        raylib.draw_rectangle(
            screen_world_zero.x as i32,
            0,
            screen_world_size.x as i32,
            5,
            config.colors.white,
        );

        // Render the player
        render_character_in_camera_space(raylib, &self.player, &config);
    }
}
