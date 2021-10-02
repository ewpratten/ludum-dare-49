use dirty_fsm::{Action, ActionFlag};
use raylib::prelude::*;

use crate::{
    character::{CharacterState, MainCharacter},
    context::GameContext,
    utilities::{
        render_layer::{FrameUpdate, ScreenSpaceRender, WorldSpaceRender},
        world_paint_texture::WorldPaintTexture,
    },
};

use self::level::Level;

use super::{Scenes, ScreenError};
use tracing::{debug, trace};

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
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for InGameScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, _context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running InGameScreen for the first time");

        // Set the player to running
        let cur_level = self.levels.get(self.current_level_idx).unwrap();
        self.player.update_player(
            Some(CharacterState::Running),
            &cur_level.colliders,
            -cur_level.platform_tex.height as f32,
        );

        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        puffin::profile_function!();
        trace!("execute() called on InGameScreen");

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

        Ok(ActionFlag::Continue)
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished InGameScreen");
        Ok(())
    }
}
