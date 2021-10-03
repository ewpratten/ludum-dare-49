use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use discord_sdk::activity::{ActivityBuilder, Assets};
use raylib::prelude::*;

use crate::{
    character::{CharacterState, MainCharacter},
    context::{ControlFlag, GameContext},
    utilities::{
        render_layer::{FrameUpdate, ScreenSpaceRender, WorldSpaceRender},
        world_paint_texture::WorldPaintTexture,
    },
};

use self::level::Level;

use super::{Scenes, ScreenError};
use tracing::{debug, error, trace};

mod hud;
pub mod level;
mod update;
pub mod world;

#[derive(Debug)]
pub struct InGameScreen {
    camera: Camera2D,
    player: MainCharacter,
    world_background: WorldPaintTexture,
    levels: Vec<Level>,
    current_level_idx: usize,
    player_dead: bool,
    level_switch_timestamp: DateTime<Utc>,
}

impl InGameScreen {
    /// Construct a new `InGameScreen`
    pub fn new(
        player_sprite_sheet: Texture2D,
        background_texture: Texture2D,
        levels: Vec<Level>,
    ) -> Self {
        Self {
            camera: Camera2D {
                offset: Vector2::zero(),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
            player: MainCharacter::new(Vector2::new(0.0, -85.0), player_sprite_sheet),
            world_background: WorldPaintTexture::new(background_texture),
            levels,
            current_level_idx: 0,
            player_dead: false,
            level_switch_timestamp: Utc::now(),
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for InGameScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running InGameScreen for the first time");

        // Handle cleanup after death
        self.player_dead = false;
        self.player.reset();

        // Set the player to running
        let cur_level = self.levels.get(context.current_level).unwrap();
        let _ = self.player.update_player(
            Some(CharacterState::Running),
            &cur_level.colliders,
            -cur_level.platform_tex.height as f32,
        );

        // Update discord
        if let Err(e) = context.discord_rpc_send.send(Some(
            ActivityBuilder::default()
                .details(format!("LVL {}", context.current_level))
                .assets(
                    Assets::default().large("game-logo-small", Some(context.config.name.clone())),
                )
                .start_timestamp(self.level_switch_timestamp),
        )) {
            error!("Failed to update discord: {}", e);
        }

        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        puffin::profile_function!();
        trace!("execute() called on InGameScreen");

        if self.current_level_idx != context.current_level {
            self.current_level_idx = context.current_level;
            self.level_switch_timestamp = Utc::now();
            context
                .flag_send
                .send(Some(ControlFlag::UpdateLevelStart(
                    self.level_switch_timestamp,
                )))
                .unwrap();
        }

        // Grab exclusive access to the renderer
        let mut renderer = context.renderer.borrow_mut();

        // Update the inputs and checking logic
        self.update(&mut renderer, delta, &context.config);

        // Wipe the background
        renderer.clear_background(context.config.colors.background);

        // Render the world
        {
            // Enter 2D mode
            let mut raylib_camera_space = renderer.begin_mode2D(self.camera);

            // Render in world space
            self.render_world_space(&mut raylib_camera_space, &context.config);
        }

        // Render the HUD
        self.render_screen_space(&mut renderer, &context.config);

        // Check if the player won
        let cur_level = self.levels.get(context.current_level).unwrap();
        if self.player.position.x > cur_level.zones.win.x {
            // Save the progress
            context
                .flag_send
                .send(Some(ControlFlag::SaveProgress))
                .unwrap();

            // If this is the last level, win the game
            if self.current_level_idx >= self.levels.len() - 1 {
                return Ok(ActionFlag::SwitchState(Scenes::WinScreen));
            } else {
                // Otherwise, increment the level counter and switch to the next level
                context
                    .flag_send
                    .send(Some(ControlFlag::SwitchLevel(self.current_level_idx + 1)))
                    .unwrap();

                // TODO: This is where the timer should reset and publish state
                return Ok(ActionFlag::SwitchState(Scenes::NextLevelScreen));
            }
        }

        if renderer.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            Ok(ActionFlag::SwitchState(Scenes::PauseScreen))
        } else if self.player_dead {
            Ok(ActionFlag::SwitchState(Scenes::DeathScreen))
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished InGameScreen");

        // Handle resetting if the player dies
        if self.player_dead {
            self.player_dead = false;
            self.player.reset();
        }

        Ok(())
    }
}
