pub mod collisions;
pub mod render;

use chrono::{DateTime, Utc};
use raylib::{math::Vector2, texture::Texture2D};

use crate::utilities::anim_render::AnimatedSpriteSheet;

use self::collisions::modify_player_based_on_forces;

#[derive(Debug, Default)]
pub enum CharacterState {
    #[default]
    Running,
    Jumping,
    Dashing,
}

#[derive(Debug)]
pub struct MainCharacter {
    pub position: Vector2,
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
            velocity: Vector2::new(20.0, 0.0),
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

    pub fn apply_force(&mut self, force: Vector2) -> Option<()> {
        self.velocity = force;
        modify_player_based_on_forces(self).unwrap();
        // self.position = calculate_player_collisions(&self).unwrap();
        Some(())
    }

    pub fn update_gravity(&mut self) {
        modify_player_based_on_forces(self).unwrap();
    }

    pub fn set_state(&mut self, state: CharacterState) {
        self.current_state = state;
        self.state_set_timestamp = Utc::now();
    }
}
