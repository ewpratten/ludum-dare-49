use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};

use crate::{context::GameContext, utilities::render_layer::ScreenSpaceRender};

use super::{Scenes, ScreenError};
use tracing::{debug, error, info, trace};

/// Defines how long the loading screen should be displayed.
const LOADING_SCREEN_DURATION_SECONDS: u8 = 3;

#[derive(Debug)]
pub struct LoadingScreen {
    start_timestamp: Option<DateTime<Utc>>,
}

impl LoadingScreen {
    /// Construct a new LoadingScreen
    pub fn new() -> Self {
        Self {
            start_timestamp: None,
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

        // Keep rendering until we pass the loading screen duration
        if let Some(start_timestamp) = self.start_timestamp {
            let duration = Utc::now().signed_duration_since(start_timestamp);
            if duration.num_seconds() >= LOADING_SCREEN_DURATION_SECONDS as i64 {
                info!("LoadingScreen duration reached, moving to next screen");
                Ok(ActionFlag::SwitchState(Scenes::FsmErrorScreen))
            } else {
                Ok(ActionFlag::Continue)
            }
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished LoadingScreen");

        // Reset the start timestamp
        self.start_timestamp = None;

        Ok(())
    }
}

impl ScreenSpaceRender for LoadingScreen {
    fn render_screen_space(
        &self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
    ) {

        // Calculate the loading screen fade in/out value
        // This makes the loading screen fade in/out over the duration of the loading screen



    }
}
