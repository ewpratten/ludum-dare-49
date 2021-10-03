use self::{
  pause_screen::PauseScreen,
    fsm_error_screen::FsmErrorScreen,
    ingame_scene::{level::loader::load_all_levels, InGameScreen},
    main_menu_screen::MainMenuScreen, options_screen::OptionsScreen, how_to_play_screen::HowToPlayScreen,
    death_screen::DeathScreen, win_screen::WinScreen,
    next_level_screen::NextLevelScreen
};
use crate::{
    context::GameContext,
    utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        non_ref_raylib::HackedRaylibHandle,
    },
};
use dirty_fsm::StateMachine;
use raylib::{texture::Texture2D, RaylibThread};

pub mod fsm_error_screen;
pub mod ingame_scene;
pub mod main_menu_screen;
pub mod how_to_play_screen;
pub mod options_screen;
pub mod pause_screen;
pub mod death_screen;
pub mod win_screen;
pub mod next_level_screen;

/// Defines all scenes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Scenes {
    #[default]
    FsmErrorScreen,
    MainMenuScreen,
    InGameScene,
    HowToPlayScreen,
    OptionsScreen,
    PauseScreen,
    DeathScreen,
    WinScreen,
    NextLevelScreen,
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
    // Load the various textures needed by the states
    let player_sprite_sheet =
        load_texture_from_internal_data(raylib_handle, thread, "character/player_run.png").unwrap();
    let world_background =
        load_texture_from_internal_data(raylib_handle, thread, "default-texture.png").unwrap();
    let levels = load_all_levels(raylib_handle, thread).unwrap();

    // Set up the state machine
    let mut machine = StateMachine::new();
    machine.add_action(Scenes::FsmErrorScreen, FsmErrorScreen::new())?;
    machine.add_action(Scenes::MainMenuScreen, MainMenuScreen::new())?;
    machine.add_action(Scenes::HowToPlayScreen, HowToPlayScreen::new())?;
    machine.add_action(Scenes::OptionsScreen, OptionsScreen::new())?;
    machine.add_action(Scenes::PauseScreen, PauseScreen::new())?;
    machine.add_action(
        Scenes::InGameScene,
        InGameScreen::new(player_sprite_sheet, world_background, levels),
    )?;
    machine.add_action(Scenes::DeathScreen, DeathScreen::new())?;
    machine.add_action(Scenes::WinScreen, WinScreen::new())?;
    machine.add_action(Scenes::NextLevelScreen, NextLevelScreen::new())?;
    Ok(machine)

}
