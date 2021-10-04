pub mod collisions;
pub mod render;

use chrono::{DateTime, Utc};
use raylib::{
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

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
    pub start_position: Vector2,
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
            start_position: position.clone(),
            position,
            movement_force: Vector2::zero(),
            velocity: Vector2::zero(),
            base_velocity: Vector2::new(0.0, GRAVITY_PPS),
            size: Vector2::new(85.0, 100.0),
            sprite_sheet: AnimatedSpriteSheet::new(
                sprite_sheet,
                Vector2::new(258.0, 277.0),
                4,
                15,
                0,
            ),
            current_state: CharacterState::default(),
            state_set_timestamp: Utc::now(),
        }
    }

    pub fn override_state(&mut self, state: CharacterState) {
        // Update the internal state
        if state != self.current_state {
            self.current_state = state.clone();
            self.state_set_timestamp = Utc::now();
        }
    }

    #[must_use]
    pub fn update_player(
        &mut self,
        state: Option<CharacterState>,
        colliders: &Vec<Rectangle>,
        killers: &Vec<Rectangle>,
        level_height_offset: f32,
    ) -> Result<(), ()> {
        if let Some(state) = state {
            // Handle extra external forces based on the character state
            self.movement_force = match &state {
                CharacterState::Running => Vector2::new(10.0, 0.0),
                CharacterState::Jumping => Vector2::new(10.0, -40.0),
                CharacterState::Dashing => Vector2::new(30.0, -20.0),
            };

            // Update the internal state
            self.override_state(state);
        }

        // Update the player based on the new velocity
        modify_player_based_on_forces(self, colliders, killers, level_height_offset)
    }

    pub fn reset(&mut self) {
        self.position = self.start_position;
        self.velocity = Vector2::zero();
        self.movement_force = Vector2::zero();
        self.current_state = CharacterState::default();
        self.state_set_timestamp = Utc::now();
    }
}
