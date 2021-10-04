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
pub struct HowToPlayScreen {
    is_btm_pressed: bool, //Is back to menu button pressed
    counter: i32,
}

impl HowToPlayScreen {
    /// Construct a new `HowToPlayScreen`
    pub fn new() -> Self {
        Self {
            is_btm_pressed: false,
            counter: 0,
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for HowToPlayScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running HowToPlayScreen for the first time");

        if let Err(e) = context.discord_rpc_send.send(Some(
            ActivityBuilder::default()
                .details("learning how to play")
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
        trace!("execute() called on HowToPlayScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        self.counter += 1;

        if self.is_btm_pressed {
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();
            Ok(ActionFlag::SwitchState(Scenes::MainMenuScreen))
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished HowToPlayScreen");
        self.is_btm_pressed = false;
        self.counter = 0;
        Ok(())
    }
}

impl ScreenSpaceRender for HowToPlayScreen {
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

        let screen_size = raylib.get_screen_size();

        //Mouse Position
        let mouse_position: Vector2 = raylib.get_mouse_position();

        let mouse_pressed: bool = raylib.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);

        //Render the title
        let timer: i32 = get_random_value(50, 400);
            if self.counter > timer {
                raylib.draw_rgb_split_text(
                    Vector2::new(40.0, 80.0),
                    "[How to Play]",
                    70,
                    true,
                    Color::WHITE,
                );
                if self.counter > timer + 20 {
                    self.counter = 0;
                }
            }
            else{
                raylib.draw_rgb_split_text(
                    Vector2::new(40.0, 80.0),
                    "[How to Play]",
                    70,
                    false,
                    Color::WHITE,
                );
            }

        // Render the instructions
        raylib.draw_rgb_split_text(
            Vector2::new(100.0, 300.0),
            ">> SPACE to jump\n>> SHIFT to dash\n>> Marcelo made these maps\n>> Marcelo hates you",
            45,
            true,
            Color::WHITE,
        );

        //Back to Menu
        let hovering_back_button = Rectangle::new(35.0, screen_size.y as f32 - 80.0, 200.0, 40.0)
            .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(25.0, screen_size.y - 50.0),
            "BACK TO MENU",
            25,
            hovering_back_button,
            Color::WHITE,
        );
        self.is_btm_pressed = hovering_back_button && mouse_pressed;
    }
}
