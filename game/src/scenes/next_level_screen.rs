use std::ops::{Div, Sub};

use chrono::{DateTime, Duration, Utc};
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
pub struct NextLevelScreen {
    is_next_pressed: bool,
    screen_load_time: DateTime<Utc>,
    attempt_time: String,
    best_time: String,
}

impl NextLevelScreen {
    /// Construct a new `NextLevelScreen`
    pub fn new() -> Self {
        Self {
            is_next_pressed: false,
            screen_load_time: Utc::now(),
            attempt_time: String::new(),
            best_time: String::new(),
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for NextLevelScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running NextLevelScreen for the first time");
        self.screen_load_time = Utc::now();

        if let Err(e) = context.discord_rpc_send.send(Some(
            ActivityBuilder::default().details("accepting fate").assets(
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
        trace!("execute() called on NextLevelScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        let attempt_elapsed = self.screen_load_time - context.level_start_time;
        self.attempt_time = format!(
            "{:02}:{:02}",
            attempt_elapsed.num_minutes(),
            attempt_elapsed.num_seconds() % 60
        );
        let best_time = context
            .player_progress
            .get_level_best_time(context.current_level)
            .unwrap_or(attempt_elapsed);
        self.best_time = format!(
            "{:02}:{:02}",
            best_time.num_minutes(),
            best_time.num_seconds() % 60
        );

        if self.is_next_pressed {
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished NextLevelScreen");
        self.is_next_pressed = false;
        Ok(())
    }
}

impl ScreenSpaceRender for NextLevelScreen {
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
            Vector2::new(80.0, screen_size.y / 2.0 - 100.0),
            "LEVEL COMPLETE",
            50,
            true,
            Color::WHITE,
        );

        //Time
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, screen_size.y / 2.0 - 40.0),
            &format!("YOUR TIME: {}", self.attempt_time),
            20,
            false,
            Color::WHITE,
        );
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, screen_size.y / 2.0 - 20.0),
            &format!("BEST TIME: {}", self.best_time),
            20,
            false,
            Color::WHITE,
        );

        //Next Level
        let hovering_next_button =
            Rectangle::new(80.0, screen_size.y as f32 / 2.0 + 50.0, 200.0, 40.0)
                .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(80.0, screen_size.y / 2.0 + 50.0),
            ">> Next Level",
            25,
            hovering_next_button,
            Color::WHITE,
        );
        self.is_next_pressed = hovering_next_button && mouse_pressed;
    }
}
