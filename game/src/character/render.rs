use std::ops::{Div, Sub};

use raylib::prelude::*;

use crate::utilities::non_ref_raylib::HackedRaylibHandle;

use super::MainCharacter;

pub fn render_character_in_camera_space(
    raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>,
    player: &MainCharacter,
) {

    raylib.draw_rectangle_v(player.position.sub(player.size.div(2.0)), player.size, Color::WHITE);
}
