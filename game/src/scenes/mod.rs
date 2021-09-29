use std::{
    borrow::Borrow,
    cell::{Cell, RefCell, RefMut},
    rc::Rc,
};

use dirty_fsm::{Action, StateMachine};
use raylib::{
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle,
};

use crate::{context::GameContext, gfx::render_layer::{FrameUpdate, ScreenSpaceRender, WorldSpaceRender}, utilities::non_ref_raylib::HackedRaylibHandle};

use self::fsm_error_screen::FsmErrorScreen;

pub mod fsm_error_screen;
// pub mod loading_screen;

/// Data passed to all scenes upon render
// pub type RenderContext<'a, 'b> = (&'b mut RaylibDrawHandle<'a>, &'b mut GameContext);

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
pub fn build_screen_state_machine() -> Result<
    // StateMachine<Scenes, ScreenError, RefCell<(NonRefDrawHandle, Rc<RefCell<GameContext>>)>>,
    StateMachine<Scenes, ScreenError, RefCell<HackedRaylibHandle>>,
    ScreenError,
> {
    let mut machine = StateMachine::new();
    machine.add_action(Scenes::FsmErrorScreen, FsmErrorScreen::new())?;
    Ok(machine)
}
