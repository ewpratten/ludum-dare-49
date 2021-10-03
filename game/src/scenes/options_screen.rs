use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
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
use tracing::{debug, info, trace};

#[derive(Debug)]
pub struct OptionsScreen {
    is_btm_pressed: bool, //Is back to menu button pressed
}

impl OptionsScreen {
    /// Construct a new `OptionsScreen`
    pub fn new() -> Self {
        Self {
            is_btm_pressed: false,
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for OptionsScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, _context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running OptionsScreen for the first time");

        // Rick-roll the user
        let _ = webbrowser::open("https://www.youtube.com/watch?v=dQw4w9WgXcQ");

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on OptionsScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        if self.is_btm_pressed {
            Ok(ActionFlag::SwitchState(Scenes::MainMenuScreen))
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished OptionsScreen");
        self.is_btm_pressed = false;
        Ok(())
    }
}

impl ScreenSpaceRender for OptionsScreen {
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

        // Render the title
        raylib.draw_rgb_split_text(Vector2::new(40.0, 80.0), "Options", 70, true, Color::WHITE);

        // Render the text
        raylib.draw_rgb_split_text(
            Vector2::new(100.0, 300.0),
            ">> The game controls YOU",
            45,
            true,
            Color::WHITE,
        );


        //Back to Menu
        let hovering_back = Rectangle::new(35.0, screen_size.y as f32 - 80.0, 200.0, 40.0)
            .check_collision_point_rec(mouse_position);
        raylib.draw_rgb_split_text(
            Vector2::new(25.0, screen_size.y - 50.0),
            "BACK TO MENU",
            25,
            hovering_back,
            Color::WHITE,
        );
        self.is_btm_pressed = mouse_pressed && hovering_back;
    }
}
