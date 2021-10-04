use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use discord_sdk::activity::{ActivityBuilder, Assets};
use pkg_version::pkg_version_major;
use raylib::prelude::*;

use crate::{GameConfig, context::{ControlFlag, GameContext}, utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        game_version::get_version_string,
        math::interpolate_exp,
        non_ref_raylib::HackedRaylibHandle,
        render_layer::ScreenSpaceRender,
    }};

use super::{Scenes, ScreenError};
use tracing::{debug, error, info, trace};

#[derive(Debug)]
pub struct WinScreen {
    is_menu_pressed: bool, //Is menu button pressed
    counter: i32,
}

impl WinScreen {
    /// Construct a new `WinScreen`
    pub fn new() -> Self {
        Self {
            is_menu_pressed: false,
            counter: 0,
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for WinScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running WinScreen for the first time");

        if let Err(e) = context.discord_rpc_send.send(Some(
            ActivityBuilder::default()
                .details("somehow won the game")
                .assets(
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
        trace!("execute() called on WinScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);
        self.counter += 1;

        if self.is_menu_pressed {
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();
            context.flag_send.send(Some(ControlFlag::EndLevel)).unwrap();
            Ok(ActionFlag::SwitchState(Scenes::MainMenuScreen))
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished WinScreen");
        self.is_menu_pressed = false;
        self.counter = 0;

        Ok(())
    }
}

impl ScreenSpaceRender for WinScreen {
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

        //Mouse Position
        let mouse_position: Vector2 = raylib.get_mouse_position();

        let mouse_pressed: bool = raylib.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);

        raylib.draw_rgb_split_text(
            Vector2::new(100.0, screen_size.y as f32 / 2.0 - 120.0),
            "congrats.",
            40,
            false,
            Color::WHITE,
        );

        if self.counter > 100 {
            raylib.draw_rgb_split_text(
                Vector2::new(100.0, screen_size.y as f32 / 2.0 - 60.0),
                "you win.",
                40,
                false,
                Color::WHITE,
            );
        }
        if self.counter > 300 {
            raylib.draw_rgb_split_text(
                Vector2::new(100.0, screen_size.y as f32 / 2.0),
                "yay.",
                40,
                false,
                Color::WHITE,
            );
        };

        //Return to Main Menu
        if self.counter > 450 {
            if Rectangle::new(100.0, screen_size.y as f32 / 2.0 + 90.0, 270.0, 20.0)
                .check_collision_point_rec(mouse_position)
            {
                raylib.draw_rgb_split_text(
                    Vector2::new(100.0, screen_size.y as f32 / 2.0 + 100.0),
                    ">> RETURN TO MAIN MENU",
                    20,
                    true,
                    Color::WHITE,
                );

                self.is_menu_pressed = mouse_pressed
            } else {
                raylib.draw_rgb_split_text(
                    Vector2::new(100.0, screen_size.y as f32 / 2.0 + 100.0),
                    ">> RETURN TO MAIN MENU",
                    20,
                    false,
                    Color::WHITE,
                );
            }
        }
    }
}
