use self::{
    fsm_error_screen::FsmErrorScreen, ingame_scene::InGameScreen, loading_screen::LoadingScreen,
    main_menu_screen::MainMenuScreen,
};
use crate::{context::GameContext, utilities::{datastore::{ResourceLoadError, load_level_from_internal_data}, non_ref_raylib::HackedRaylibHandle}};
use dirty_fsm::StateMachine;
use raylib::RaylibThread;

pub mod fsm_error_screen;
pub mod ingame_scene;
pub mod loading_screen;
pub mod main_menu_screen;

/// Defines all scenes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Scenes {
    #[default]
    FsmErrorScreen,
    LoadingScreen,
    MainMenuScreen,
    InGameScene,
}

/// Contains any possible errors thrown while rendering
#[derive(Debug, Error)]
pub enum ScreenError {
    #[error(transparent)]
    ResourceLoad(#[from] ResourceLoadError),
}

/// Build the state machine for all scenes
pub fn build_screen_state_machine(
    raylib_handle: &mut HackedRaylibHandle,
    thread: &RaylibThread,
) -> Result<
    // StateMachine<Scenes, ScreenError, RefCell<(NonRefDrawHandle, Rc<RefCell<GameContext>>)>>,
    StateMachine<Scenes, ScreenError, GameContext>,
    ScreenError,
> {

    let levels = vec![load_level_from_internal_data( raylib_handle, &thread, "levels_tmx/test_map.tmx", "levels_tmx/tilemap1.tsx").expect("Could not load test level.")];

    let mut machine = StateMachine::new();
    machine.add_action(Scenes::FsmErrorScreen, FsmErrorScreen::new())?;
    machine.add_action(
        Scenes::LoadingScreen,
        LoadingScreen::new(raylib_handle, thread)?,
    )?;
    machine.add_action(Scenes::MainMenuScreen, MainMenuScreen::new())?;
    machine.add_action(Scenes::InGameScene, InGameScreen::new(levels))?;
    Ok(machine)
}
