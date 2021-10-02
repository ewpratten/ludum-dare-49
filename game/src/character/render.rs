use std::ops::{Div, Sub};

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
    let frames_since_state_change = ((time_since_state_change.num_milliseconds() as f64 / 1000.0) * config.animation_fps as f64) as f32;

    trace!(
        "Rendering player frame: {} ({})",
        frames_since_state_change % player.sprite_sheet.sprite_count as f32,
        frames_since_state_change
    );
    player.sprite_sheet.render(
        raylib,
        player.position.sub(player.size.div(2.0)),
        Some(Vector2::new(player.size.y, player.size.y)),
        Some((frames_since_state_change % player.sprite_sheet.sprite_count as f32).floor() as usize),
    );
}
