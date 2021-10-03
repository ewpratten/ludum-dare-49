use std::ops::Mul;

use raylib::math::{Rectangle, Vector2};

use crate::scenes::ingame_scene::world::WORLD_LEVEL_X_OFFSET;

use super::{CharacterState, MainCharacter};

pub const GRAVITY_PPS: f32 = 2.0;

#[must_use]
pub fn modify_player_based_on_forces(
    player: &mut MainCharacter,
    colliders: &Vec<Rectangle>,
    level_height_offset: f32,
) -> Result<(), ()> {
    // Modify the player's velocity by the forces
    player.movement_force += player.base_velocity;
    player.velocity = player.movement_force;

    // Predict the player's position next frame
    let predicted_player_position = player.position + player.velocity;

    // Calculate a bounding rect around the player both now, and one frame in the future
    let player_rect = Rectangle::new(
        predicted_player_position.x - (player.size.x / 2.0),
        predicted_player_position.y - (player.size.x / 2.0),
        player.size.x,
        player.size.y,
    );

    // Calculate a generic "floor" to always collide with
    let floor_rect = Rectangle::new(f32::MIN, 0.0, f32::MAX, 1.0);

    // Check collision conditions
    let check_player_colliding_with_floor = || floor_rect.check_collision_recs(&player_rect);
    let check_player_colliding_with_floor_next_frame =
        || player_rect.y + player_rect.height > floor_rect.y;
    let check_player_colliding_with_colliders = || {
        colliders.iter().any(|rect| {
            let mut translated_rect = rect.clone();
            translated_rect.y += level_height_offset;
            translated_rect.x += WORLD_LEVEL_X_OFFSET;
            translated_rect.check_collision_recs(&player_rect)
        })
    };
    let check_player_colliding_with_colliders_forwards = || {
        colliders.iter().any(|rect| {
            let mut translated_rect = rect.clone();
            translated_rect.y += level_height_offset;
            translated_rect.x += WORLD_LEVEL_X_OFFSET;
            translated_rect.check_collision_recs(&Rectangle{
                x: player_rect.x + 1.0,
                y: player_rect.y - 1.0 ,
                width: player_rect.width,
                height: player_rect.height,
            })
        })
    };

    // If the player is colliding, only apply the x force
    if (check_player_colliding_with_floor()
        || check_player_colliding_with_floor_next_frame()
        || check_player_colliding_with_colliders())
        && player.velocity.y != 0.0
    {
        player.velocity.y = 0.0;

        // Handle ending a jump
        if player.current_state == CharacterState::Jumping
            || player.current_state == CharacterState::Dashing
        {
            return player.update_player(
                Some(CharacterState::Running),
                colliders,
                level_height_offset,
            );
        }
    }

    // Check sideways collisions
    if player.velocity.y == 0.0 && check_player_colliding_with_colliders_forwards(){
        return Err(());
    }

    // Finally apply the velocity to the player
    player.position += player.velocity;

    Ok(())
}
