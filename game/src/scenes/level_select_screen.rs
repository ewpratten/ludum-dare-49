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
pub struct LevelSelectScreen {
    is_level_one_pressed: bool, //Is back to menu button pressed
    is_btm_pressed: bool,
}

impl LevelSelectScreen {
    /// Construct a new `LevelSelectScreen`
    pub fn new() -> Self {
        Self {
            is_level_one_pressed: false,
            is_btm_pressed: false,
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for LevelSelectScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running LevelSelectScreen for the first time");

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
        trace!("execute() called on LevelSelectScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        if self.is_level_one_pressed {
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        }
        else if self.is_btm_pressed {
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
        debug!("Finished LevelSelectScreen");
        self.is_level_one_pressed = false;
        self.is_btm_pressed = false;
        Ok(())
    }
}

impl ScreenSpaceRender for LevelSelectScreen {
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
        raylib.draw_rgb_split_text(
            Vector2::new(40.0, 80.0),
            "Level Select",
            70,
            true,
            Color::WHITE,
        );

        // Render the levels
        let hovering_level_one_button = Rectangle::new(100.0, 300.0, 180.0, 20.0)
            .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(100.0, 300.0),
            "LEVEL ONE",
            25,
            hovering_level_one_button,
            Color::WHITE,
        );
        if hovering_level_one_button {
            raylib.draw_rgb_split_text(
                Vector2::new(70.0, 300.0),
                ">>",
                25,
                hovering_level_one_button,
                Color::WHITE,
            );
        };
        self.is_level_one_pressed = mouse_pressed && hovering_level_one_button;

        let hovering_level_two_button = Rectangle::new(100.0, 350.0, 180.0, 20.0)
            .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(100.0, 350.0),
            "LEVEL TWO",
            25,
            hovering_level_two_button,
            Color::WHITE,
        );
        if hovering_level_two_button {
            raylib.draw_rgb_split_text(
                Vector2::new(70.0, 350.0),
                ">>",
                25,
                hovering_level_two_button,
                Color::WHITE,
            );
        };

        let hovering_level_three_button = Rectangle::new(100.0, 400.0, 210.0, 20.0)
            .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(100.0, 400.0),
            "LEVEL THREE",
            25,
            hovering_level_three_button,
            Color::WHITE,
        );
        if hovering_level_three_button {
            raylib.draw_rgb_split_text(
                Vector2::new(70.0, 400.0),
                ">>",
                25,
                hovering_level_three_button,
                Color::WHITE,
            );
        };

        let hovering_level_four_button = Rectangle::new(100.0, 450.0, 200.0, 20.0)
            .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(100.0, 450.0),
            "LEVEL FOUR",
            25,
            hovering_level_four_button,
            Color::WHITE,
        );
        if hovering_level_four_button {
            raylib.draw_rgb_split_text(
                Vector2::new(70.0, 450.0),
                ">>",
                25,
                hovering_level_four_button,
                Color::WHITE,
            );
        };

        let hovering_level_five_button = Rectangle::new(100.0, 500.0, 200.0, 20.0)
            .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(100.0, 500.0),
            "LEVEL FIVE",
            25,
            hovering_level_five_button,
            Color::WHITE,
        );
        if hovering_level_five_button {
            raylib.draw_rgb_split_text(
                Vector2::new(70.0, 500.0),
                ">>",
                25,
                hovering_level_five_button,
                Color::WHITE,
            );
        };

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
