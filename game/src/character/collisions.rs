use std::ops::Mul;

use raylib::math::{Rectangle, Vector2};

use super::{CharacterState, MainCharacter};

pub const GRAVITY_PPS: f32 = 2.0;

pub fn modify_player_based_on_forces(player: &mut MainCharacter) -> Result<(), ()> {
    // Modify the player's velocity by the forces
    player.movement_force += player.base_velocity;
    player.velocity = player.movement_force;

    // Predict the player's position next frame
    let predicted_player_position = player.position + player.velocity;

    // Calculate a bounding rect around the player
    let player_rect = Rectangle::new(
        predicted_player_position.x - (player.size.x / 2.0),
        predicted_player_position.y - (player.size.x / 2.0),
        player.size.x,
        player.size.y,
    );

    // Calculate a generic "floor" to always collide with
    let floor_rect = Rectangle::new(f32::MIN, 0.0, f32::MAX, 1.0);

    // If the player is colliding, only apply the x force
    if (floor_rect.check_collision_recs(&player_rect)
        || player_rect.y + player_rect.height > floor_rect.y)
        && player.velocity.y > 0.0
    {
        player.velocity.y = 0.0;

        // Handle ending a jump
        if player.current_state == CharacterState::Jumping
            || player.current_state == CharacterState::Dashing
        {
            player.update_player(Some(CharacterState::Running));
            return Ok(());
        }
    }

    // Finally apply the velocity to the player
    player.position += player.velocity;

    Ok(())
}
