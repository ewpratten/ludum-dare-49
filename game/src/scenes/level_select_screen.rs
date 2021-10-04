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
pub struct LevelSelectScreen {
    is_btm_pressed: bool,
    selected_level: Option<usize>,
    visible_levels: usize,
}

impl LevelSelectScreen {
    /// Construct a new `LevelSelectScreen`
    pub fn new() -> Self {
        Self {
            is_btm_pressed: false,
            selected_level: None,
            visible_levels: 0,
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

        // Calculate the number of levels to render
        self.visible_levels =
            (context.player_progress.level_best_times.len() + 1).min(context.total_levels);

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on LevelSelectScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        if let Some(level) = self.selected_level {
            // Play the sound
            context
                .flag_send
                .send(Some(ControlFlag::SoundTrigger("button-press".to_string())))
                .unwrap();

            // Switch the level
            context
                .flag_send
                .send(Some(ControlFlag::SwitchLevel(level)))
                .unwrap();

            context
                .flag_send
                .send(Some(ControlFlag::UpdateLevelStart(
                    Utc::now(),
                )))
                .unwrap();

            // Enter the game
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        } else if self.is_btm_pressed {
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
        self.selected_level = None;
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
        for level in 0..self.visible_levels {
            let hovering_button =
                Rectangle::new(100.0, 300.0 + (25.0 * level as f32), 180.0, 25.0 ).check_collision_point_rec(mouse_position);
            raylib.draw_rgb_split_text(
                Vector2::new(100.0, 300.0+ (25.0 * level as f32)),
                &format!("LEVEL {}", level),
                25,
                hovering_button,
                Color::WHITE,
            );
            if hovering_button {
                raylib.draw_rgb_split_text(
                    Vector2::new(70.0, 300.0+ (25.0 * level as f32)),
                    ">>",
                    25,
                    hovering_button,
                    Color::WHITE,
                );
            };
            if mouse_pressed && hovering_button {
                self.selected_level = Some(level);
                break;
            }
        }

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
