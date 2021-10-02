use dirty_fsm::{Action, ActionFlag};
use raylib::prelude::*;

use crate::{
    character::MainCharacter,
    context::GameContext,
    utilities::render_layer::{FrameUpdate, ScreenSpaceRender, WorldSpaceRender},
};

use super::{Scenes, ScreenError};
use tracing::{debug, trace};

mod hud;
mod update;
mod world;

#[derive(Debug)]
pub struct InGameScreen {
    camera: Camera2D,
    player: MainCharacter,
}

impl InGameScreen {
    /// Construct a new `InGameScreen`
    pub fn new() -> Self {
        Self {
            camera: Camera2D {
                offset: Vector2::zero(),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
            player: MainCharacter::new(Vector2::new(0.0, -45.0)),
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

        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on InGameScreen");

        // Grab exclusive access to the renderer
        let mut renderer = context.renderer.borrow_mut();

        // Update the inputs and checking logic
        self.update(&mut renderer, delta);

        // Wipe the background
        renderer.clear_background(Color::BLACK);

        // Render the world
        {
            // Enter 2D mode
            let mut raylib_camera_space = renderer.begin_mode2D(self.camera);

            // Render in world space
            self.render_world_space(&mut raylib_camera_space);
        }

        // Render the HUD
        self.render_screen_space(&mut renderer);

        Ok(ActionFlag::Continue)
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished InGameScreen");
        Ok(())
    }
}
