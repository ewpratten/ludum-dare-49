use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};

use crate::{context::GameContext, gfx::render_layer::ScreenSpaceRender};

use super::{Scenes, ScreenError};
use tracing::{debug, error, info, trace};

#[derive(Debug)]
pub struct LoadingScreen {
    start_timestamp: Option<DateTime<Utc>>
}

impl LoadingScreen {
    /// Construct a new LoadingScreen
    pub fn new() -> Self {
        Self {
            start_timestamp: None
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for LoadingScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running LoadingScreen for the first time");

        // Keep track of when this screen is opened
        self.start_timestamp = Some(Utc::now());

        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on LoadingScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut());
        Ok(ActionFlag::Continue)
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished LoadingScreen");

        // Reset the start timestamp
        self.start_timestamp = None;

        Ok(())
    }
}

impl ScreenSpaceRender for LoadingScreen {
    fn render_screen_space(&self, raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle) {
        todo!()
    }
}
