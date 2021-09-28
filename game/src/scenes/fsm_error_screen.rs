use std::cell::RefCell;

use dirty_fsm::{Action, ActionFlag};
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use tracing::{debug, error, info, trace};

use crate::{context::GameContext, gfx::render_layer::ScreenSpaceRender};

use super::{RenderContext, Scenes, ScreenError};

#[derive(Debug)]
pub struct FsmErrorScreen {}

impl FsmErrorScreen {
    /// Construct a new FsmErrorScreen
    pub fn new() -> Self {
        Self {}
    }
}

impl<Rl> Action<Scenes, ScreenError, RefCell<(RefCell<Rl>, RefCell<GameContext>)>> for FsmErrorScreen
where
    Rl: RaylibDraw,
{
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &RefCell<(RefCell<Rl>, RefCell<GameContext>)>) -> Result<(), ScreenError> {
        debug!("Running FsmErrorScreen for the first time");
        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &RefCell<(RefCell<Rl>, RefCell<GameContext>)>,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on FsmErrorScreen, but we have not logic");
        Ok(ActionFlag::Continue)
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished FsmErrorScreen");
        Ok(())
    }
}

impl ScreenSpaceRender for FsmErrorScreen {
    fn render_screen_space(&self, raylib: &mut raylib::prelude::RaylibDrawHandle) {
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
