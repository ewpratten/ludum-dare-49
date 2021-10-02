use std::ops::{Add, Div, Mul, Sub};

use chrono::Utc;
use raylib::prelude::*;
use tracing::log::trace;

use crate::{utilities::non_ref_raylib::HackedRaylibHandle, GameConfig};

use super::MainCharacter;

pub fn render_character_in_camera_space(
    raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>,
    player: &MainCharacter,
    config: &GameConfig,
) {
    // Calculate the time since the start of the state
    let time_since_state_change = Utc::now() - player.state_set_timestamp;

    // Calculate the number of frames since state change
    let frames_since_state_change = ((time_since_state_change.num_milliseconds() as f64 / 1000.0)
        * config.animation_fps as f64) as f32;

    // Calculate the frame ID to render
    let frame_id = match player.current_state {
        crate::character::CharacterState::Jumping => 4,
        _ => (frames_since_state_change % player.sprite_sheet.sprite_count as f32).floor() as usize,
    };

    trace!(
        "Rendering player frame: {} ({})",
        frame_id,
        frames_since_state_change
    );
    player.sprite_sheet.render(
        raylib,
        player.position.sub(player.size.div(2.0)),
        Some(Vector2::new(player.size.y, player.size.y)),
        Some(frame_id),
    );

    // Possibly render a debug vector
    if config.debug_view {
        raylib.draw_line_v(
            player.position.sub(player.size.div(2.0)),
            player
                .position
                .sub(player.size.div(2.0))
                .add(player.velocity.mul(10.0).add(Vector2::new(0.0, 100.0))),
            Color::RED,
        );
    }
}
