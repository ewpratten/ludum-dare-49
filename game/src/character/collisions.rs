use raylib::math::{Rectangle, Vector2};

use super::MainCharacter;

const GRAVITY_PPS: f32 = 2.0;

pub fn modify_player_based_on_forces(player: &mut MainCharacter) -> Result<(), ()> {
    // Convert the player to a rectangle
    let predicted_player_position = player.position + player.velocity;
    let player_rect = Rectangle::new(
        predicted_player_position.x - (player.size.x / 2.0),
        predicted_player_position.y - (player.size.x / 2.0),
        player.size.x,
        player.size.y,
    );

    // Calculate a generic "floor" to always collide with
    let floor_rect = Rectangle::new(f32::MIN, 0.0, f32::MAX, 1.0);

    // If the player is colliding, only apply the x force
    if (floor_rect.check_collision_recs(&player_rect) || player_rect.y + player_rect.height > floor_rect.y)
        && player.velocity.y > 0.0
    {
        player.velocity.y = 0.0;
    }

    // TODO: Error out if colliding in the X direction

    // Apply the force
    player.position += player.velocity;

    // Apply gravity
    player.velocity.y += GRAVITY_PPS;
    Ok(())
}
