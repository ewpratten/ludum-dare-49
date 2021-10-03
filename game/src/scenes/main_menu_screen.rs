use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use discord_sdk::activity::{ActivityBuilder, Assets};
use pkg_version::pkg_version_major;
use raylib::prelude::*;

use crate::{
    context::{ControlFlag, GameContext},
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
    is_start_pressed: bool,   //Is start button pressed
    is_htp_pressed: bool,     //Is how to play button pressed
    is_options_pressed: bool, //Is options button pressed
    is_quit_pressed: bool,    //Is quit button pressed
}

impl MainMenuScreen {
    /// Construct a new `MainMenuScreen`
    pub fn new() -> Self {
        Self {
            is_start_pressed: false,
            is_htp_pressed: false,
            is_options_pressed: false,
            is_quit_pressed: false,
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
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        } else if self.is_htp_pressed {
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();
            Ok(ActionFlag::SwitchState(Scenes::HowToPlayScreen))
        } else if self.is_options_pressed {
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();
            Ok(ActionFlag::SwitchState(Scenes::OptionsScreen))
        } else if self.is_quit_pressed {
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();
            context.flag_send.send(Some(ControlFlag::Quit)).unwrap();
            Ok(ActionFlag::Continue)
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished MainMenuScreen");
        self.is_start_pressed = false;
        self.is_htp_pressed = false;
        self.is_options_pressed = false;
        self.is_quit_pressed = false;
        Ok(())
    }
}

impl ScreenSpaceRender for MainMenuScreen {
    fn render_screen_space(
        &mut self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig,
    ) {
        let screen_size = raylib.get_screen_size();

        // Render the background
        raylib.clear_background(Color::BLACK);
        raylib.draw_rectangle_lines(
            0,
            0,
            screen_size.x as i32,
            screen_size.y as i32,
            config.colors.white,
        );

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

        // Render the title
        raylib.draw_rgb_split_text(
            Vector2::new(37.0, 80.0),
            &format!("[{}]", config.name),
            70,
            true,
            Color::WHITE,
        );

        // Start Game
        let hovering_start_game =
            Rectangle::new(80.0, 300.0, 170.0, 20.0).check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, 300.0),
            "START GAME",
            25,
            hovering_start_game,
            Color::WHITE,
        );

        if hovering_start_game {
            raylib.draw_rgb_split_text(
                Vector2::new(50.0, 300.0),
                ">>",
                25,
                hovering_start_game,
                Color::WHITE,
            );
        };

        self.is_start_pressed = mouse_pressed && hovering_start_game;

        // How to Play
        let hovering_htp =
            Rectangle::new(80.0, 350.0, 170.0, 20.0).check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, 350.0),
            "HOW TO PLAY",
            25,
            hovering_htp,
            Color::WHITE,
        );
        if hovering_htp {
            raylib.draw_rgb_split_text(
                Vector2::new(50.0, 350.0),
                ">>",
                25,
                hovering_htp,
                Color::WHITE,
            );
        };
        self.is_htp_pressed = mouse_pressed && hovering_htp;

        // OPTIONS
        let hovering_options =
            Rectangle::new(80.0, 400.0, 135.0, 20.0).check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, 400.0),
            "OPTIONS",
            25,
            hovering_options,
            Color::WHITE,
        );
        if hovering_options {
            raylib.draw_rgb_split_text(
                Vector2::new(50.0, 400.0),
                ">>",
                25,
                hovering_options,
                Color::WHITE,
            );
        };
        self.is_options_pressed = mouse_pressed && hovering_options;

        // CREDITS
        let hovering_credits =
            Rectangle::new(80.0, 445.0, 135.0, 20.0).check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, 450.0),
            "CREDITS",
            25,
            hovering_credits,
            Color::WHITE,
        );
        if hovering_credits {
            raylib.draw_rgb_split_text(Vector2::new(50.0, 450.0), ">>", 25, true, Color::WHITE);
        };
        if hovering_credits && mouse_pressed {
            let _ = webbrowser::open("https://github.com/Ewpratten/ludum-dare-49#the-team");
        }

        // QUIT
        let hovering_quit =
            Rectangle::new(80.0, 495.0, 65.0, 20.0).check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, 500.0),
            "QUIT",
            25,
            hovering_quit,
            Color::WHITE,
        );
        if hovering_quit {
            raylib.draw_rgb_split_text(
                Vector2::new(50.0, 500.0),
                ">>",
                25,
                hovering_quit,
                Color::WHITE,
            );
        };
        self.is_quit_pressed = mouse_pressed && hovering_quit;

        // for
    }
}
