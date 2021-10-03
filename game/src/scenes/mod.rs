use self::{death_screen::DeathScreen, fsm_error_screen::FsmErrorScreen, how_to_play_screen::HowToPlayScreen, ingame_scene::{InGameScreen, level::{Level, loader::load_all_levels}}, level_select_screen::LevelSelectScreen, main_menu_screen::MainMenuScreen, next_level_screen::NextLevelScreen, options_screen::OptionsScreen, pause_screen::PauseScreen, win_screen::WinScreen};
use crate::{context::GameContext, utilities::{datastore::{ResourceLoadError, load_music_from_internal_data, load_sound_from_internal_data, load_texture_from_internal_data}, non_ref_raylib::HackedRaylibHandle}};
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
pub mod level_select_screen;

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
    LevelSelectScreen,
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
    levels: Vec<Level>
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
    machine.add_action(Scenes::LevelSelectScreen, LevelSelectScreen::new())?;
    Ok(machine)

}
