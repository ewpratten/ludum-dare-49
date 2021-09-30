use dirty_fsm::StateMachine;
use raylib::RaylibThread;
use crate::{context::GameContext, utilities::non_ref_raylib::HackedRaylibHandle};
use self::{fsm_error_screen::FsmErrorScreen, loading_screen::LoadingScreen};

pub mod fsm_error_screen;
pub mod loading_screen;

/// Defines all scenes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Scenes {
    #[default]
    FsmErrorScreen,
    LoadingScreen,
}

/// Contains any possible errors thrown while rendering
#[derive(Debug, Error)]
pub enum ScreenError {}

/// Build the state machine for all scenes
pub fn build_screen_state_machine(raylib_handle: &HackedRaylibHandle, thread: &RaylibThread) -> Result<
    // StateMachine<Scenes, ScreenError, RefCell<(NonRefDrawHandle, Rc<RefCell<GameContext>>)>>,
    StateMachine<Scenes, ScreenError, GameContext>,
    ScreenError,
> {
    let mut machine = StateMachine::new();
    machine.add_action(Scenes::FsmErrorScreen, FsmErrorScreen::new())?;
    machine.add_action(Scenes::LoadingScreen, LoadingScreen::new(raylib_handle, thread))?;
    Ok(machine)
}
