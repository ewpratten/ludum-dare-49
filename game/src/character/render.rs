use raylib::prelude::*;

use crate::utilities::non_ref_raylib::HackedRaylibHandle;

use super::MainCharacter;

pub fn render_character_in_camera_space(
    raylib: &mut RaylibMode2D<'_, HackedRaylibHandle>,
    player: &MainCharacter,
) {

    raylib.draw_rectangle_v(player.position, Vector2::new(10.0, 20.0), Color::WHITE);
}
