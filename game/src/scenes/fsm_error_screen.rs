use std::cell::{Cell, RefCell};

use dirty_fsm::{Action, ActionFlag};
use raylib::{color::Color, prelude::RaylibDraw, RaylibHandle};
use tracing::{debug, error, info, trace};

use crate::{context::GameContext, gfx::render_layer::ScreenSpaceRender, utilities::non_ref_raylib::HackedRaylibHandle};

use super::{Scenes, ScreenError};

#[derive(Debug)]
pub struct FsmErrorScreen {}

impl FsmErrorScreen {
    /// Construct a new FsmErrorScreen
    pub fn new() -> Self {
        Self {}
    }
}

impl Action<Scenes, ScreenError, GameContext> for FsmErrorScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running FsmErrorScreen for the first time");
        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on FsmErrorScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut());
        Ok(ActionFlag::Continue)
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished FsmErrorScreen");
        Ok(())
    }
}

impl ScreenSpaceRender for FsmErrorScreen {
    fn render_screen_space(&self, raylib: &mut HackedRaylibHandle) {
        raylib.clear_background(Color::RED);

        // Render a warning message
        raylib.draw_text(
            "FSM Failure\nFalling back to Default state",
            10,
            10,
            40,
            Color::WHITE,
        )
    }
}
