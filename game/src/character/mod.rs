pub mod collisions;
pub mod render;

use chrono::{DateTime, Utc};
use raylib::{math::Vector2, texture::Texture2D};

use crate::utilities::anim_render::AnimatedSpriteSheet;

use self::collisions::{modify_player_based_on_forces, GRAVITY_PPS};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum CharacterState {
    #[default]
    Running,
    Jumping,
    Dashing,
}

#[derive(Debug)]
pub struct MainCharacter {
    pub position: Vector2,
    pub movement_force: Vector2,
    pub base_velocity: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
    pub sprite_sheet: AnimatedSpriteSheet,
    pub current_state: CharacterState,
    pub state_set_timestamp: DateTime<Utc>,
}

impl MainCharacter {
    pub fn new(position: Vector2, sprite_sheet: Texture2D) -> Self {
        Self {
            position,
            movement_force: Vector2::zero(),
            velocity: Vector2::zero(),
            base_velocity: Vector2::new(0.0, GRAVITY_PPS),
            size: Vector2::new(100.0, 130.0),
            sprite_sheet: AnimatedSpriteSheet::new(
                sprite_sheet,
                Vector2::new(300.0, 300.0),
                3,
                8,
                6,
            ),
            current_state: CharacterState::default(),
            state_set_timestamp: Utc::now(),
        }
    }

    pub fn update_player(&mut self, state: Option<CharacterState>) {
        if let Some(state) = state {
            // Update the internal state
            if state != self.current_state {
                self.current_state = state.clone();
                self.state_set_timestamp = Utc::now();
            }

            // Handle extra external forces based on the character state
            self.movement_force = match state {
                CharacterState::Running => Vector2::new(12.0, 0.0),
                CharacterState::Jumping => Vector2::new(12.0, -30.0),
                CharacterState::Dashing => Vector2::new(30.0, -20.0),
            };
        }

        // Update the player based on the new velocity
        modify_player_based_on_forces(self).unwrap();
    }
}
