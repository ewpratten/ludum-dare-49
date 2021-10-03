use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use pkg_version::pkg_version_major;
use raylib::prelude::*;

use crate::{GameConfig, context::GameContext, utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        game_version::get_version_string,
        math::interpolate_exp,
        non_ref_raylib::HackedRaylibHandle,
        render_layer::ScreenSpaceRender,
    }};

use super::{Scenes, ScreenError};
use tracing::{debug, info, trace};

#[derive(Debug)]
pub struct DeathScreen {
    is_retry_pressed: bool
}

impl DeathScreen {
    /// Construct a new `DeathScreen`
    pub fn new() -> Self {
        Self {
            is_retry_pressed: false
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for DeathScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, _context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running DeathScreen for the first time");

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on DeathScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);


        if self.is_retry_pressed {
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        }
        else{
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished DeathScreen");
        self.is_retry_pressed = false;
        Ok(())
    }
}

impl ScreenSpaceRender for DeathScreen {
    fn render_screen_space(
        &mut self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig
    ) {

        // Render the background
        raylib.clear_background(Color::DARKBLUE);

        let screen_size = raylib.get_screen_size();

        //Mouse Position
        let mouse_position: Vector2 = raylib.get_mouse_position();

        let mouse_pressed: bool = raylib.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);

        raylib.draw_text(

            "ERR: Corrupted Player Data Detected

The program has detected lowering player integrity,
and has halted as a safety precaution.

If this is the first time you've seen this error screen,
restart the level. If problems continue, simply get good.

--------   Technical information   --------
*** CALL STACK:
*** C  [libraylib.so+0x75c] END_DRAWING()
*** RS [data_loss.so+0x48f] validate_player()
*** ---------------------------------------
*** PROGRAM_HALT (TIME: XX:XX, BEST: XX:XX)
*** ---------------------------------------",
            25,
            20,
            20,
            Color::WHITE,
        );

        //Retry
        if Rectangle::new(35.0, screen_size.y as f32 - 80.0, 200.0, 40.0).check_collision_point_rec(mouse_position){
            raylib.draw_text(

                ">>CLICK HERE TO RETRY",
                20,
                screen_size.y as i32 - 40,
                19,
                Color::WHITE,
            );

            if mouse_pressed{
                self.is_retry_pressed = true;

            }
        }
        else {
            raylib.draw_text(

                ">>CLICK HERE TO RETRY",
                25,
                screen_size.y as i32 - 40,
                18,
                Color::WHITE,
            );
        }
    }
}
