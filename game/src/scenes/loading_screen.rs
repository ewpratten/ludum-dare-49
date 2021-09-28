use dirty_fsm::Action;

use crate::context::GameContext;

use super::{Scenes, ScreenError};

#[derive(Debug)]
pub struct LoadingScreen {

}

impl Action<Scenes, ScreenError, GameContext> for LoadingScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        todo!()
    }

    fn on_first_run(&mut self, context: &mut GameContext) -> Result<(), ScreenError> {
        todo!()
    }

    fn execute(&mut self, delta: &chrono::Duration, context: &mut GameContext) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        todo!()
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ScreenError> {
        todo!()
    }
}
