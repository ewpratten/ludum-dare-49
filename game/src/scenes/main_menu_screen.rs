use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use discord_sdk::activity::{ActivityBuilder, Assets};
use pkg_version::pkg_version_major;
use raylib::prelude::*;

use crate::{
    context::GameContext,
    utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        game_version::get_version_string,
        math::interpolate_exp,
        non_ref_raylib::HackedRaylibHandle,
        render_layer::ScreenSpaceRender,
    },
    GameConfig,
};

use super::{Scenes, ScreenError};
use tracing::{debug, error, info, trace};

#[derive(Debug)]
pub struct MainMenuScreen {

    is_start_pressed: bool, //Is start button pressed
    is_htp_pressed: bool, //Is how to play button pressed
    is_options_pressed: bool //Is options button pressed

}

impl MainMenuScreen {
    /// Construct a new `MainMenuScreen`
    pub fn new() -> Self {
        Self {
            is_start_pressed: false,
            is_htp_pressed: false,
            is_options_pressed: false
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for MainMenuScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running MainMenuScreen for the first time");

        // Update discord
        if let Err(e) = context.discord_rpc_send.send(Some(
            ActivityBuilder::default().details("main menu").assets(
                Assets::default().large("game-logo-small", Some(context.config.name.clone())),
            ),
        )) {
            error!("Failed to update discord: {}", e);
        }

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on MainMenuScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        if self.is_start_pressed {
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        }
        else if self.is_htp_pressed {
            Ok(ActionFlag::SwitchState(Scenes::HowToPlayScreen))
        }
        else if self.is_options_pressed {
            Ok(ActionFlag::SwitchState(Scenes::OptionsScreen))
        }
        else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished MainMenuScreen");
        self.is_start_pressed = false;
        self.is_htp_pressed = false;
        self.is_options_pressed = false;
        Ok(())
    }
}

impl ScreenSpaceRender for MainMenuScreen {
    fn render_screen_space(
        &mut self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig,
    ) {
        // Render the background
        raylib.clear_background(Color::BLACK);

        // Calculate the logo position
        let screen_size = raylib.get_screen_size();

        //Mouse Position
        let mouse_position: Vector2 = raylib.get_mouse_position();

        let mouse_pressed: bool = raylib.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);

        // Only in debug mode, render a debug message
        #[cfg(debug_assertions)]
        {
            raylib.draw_text(
                "Game in DEBUG MODE. Do not redistribute!",
                10,
                screen_size.y as i32 - 35,
                15,
                Color::WHITE,
            );
        }

        // Displays mouse position
        raylib.draw_text(
            &format!("[{}, {}]", mouse_position.x, mouse_position.y),
            screen_size.x as i32 - 130,
            screen_size.y as i32 - 30,
            25,
            Color::DARKGRAY,
        );

        // Render the game version info
        raylib.draw_text(
            &format!(
                "Version: {} Commit: {}",
                get_version_string(),
                env!("VERGEN_GIT_SHA_SHORT")
            ),
            10,
            screen_size.y as i32 - 20,
            15,
            Color::WHITE,
        );

        raylib.draw_text(

            &format!("[{}]", config.name),
            37,
            80,
            70,
            Color::BLUE,
        );

        raylib.draw_text(

            &format!("[{}]", config.name),
            43,
            80,
            70,
            Color::RED,
        );

        raylib.draw_text(

            &format!("[{}]", config.name),
            40,
            80,
            70,
            Color::WHITE,
        );

        // Start Game
        if Rectangle::new(80.0, 300.0, 170.0, 20.0).check_collision_point_rec(mouse_position) {
            raylib.draw_text(

                "START GAME",
                83,
                300,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "START GAME",
                77,
                300,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "START GAME",
                80,
                300,
                25,
                Color::WHITE,
            );

            if mouse_pressed{
                self.is_start_pressed = true;
            }
        }
        else{
            raylib.draw_text(

                "START GAME",
                81,
                300,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "START GAME",
                79,
                300,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "START GAME",
                80,
                300,
                25,
                Color::WHITE,
            );
        }

        // How to Play
        if Rectangle::new(80.0, 350.0, 170.0, 20.0).check_collision_point_rec(mouse_position) {
            raylib.draw_text(

                "HOW TO PLAY",
                83,
                350,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "HOW TO PLAY",
                77,
                350,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "HOW TO PLAY",
                80,
                350,
                25,
                Color::WHITE,
            );

            if mouse_pressed{
                self.is_htp_pressed = true;
            }
        }
        else{
            raylib.draw_text(

                "HOW TO PLAY",
                81,
                350,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "HOW TO PLAY",
                79,
                350,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "HOW TO PLAY",
                80,
                350,
                25,
                Color::WHITE,
            );
        }

        // OPTIONS
        if Rectangle::new(80.0, 400.0, 135.0, 20.0).check_collision_point_rec(mouse_position) {
            raylib.draw_text(

                "OPTIONS",
                83,
                400,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "OPTIONS",
                77,
                400,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "OPTIONS",
                80,
                400,
                25,
                Color::WHITE,
            );

            if mouse_pressed{
                self.is_options_pressed = true;
            }

        }
        else{
            raylib.draw_text(

                "OPTIONS",
                81,
                400,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "OPTIONS",
                79,
                400,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "OPTIONS",
                80,
                400,
                25,
                Color::WHITE,
            );
        }
    }
}
