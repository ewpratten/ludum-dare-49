use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use pkg_version::pkg_version_major;
use raylib::prelude::*;

use crate::{GameConfig, context::GameContext, utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        game_version::get_version_string,
        math::interpolate_exp,
        non_ref_raylib::HackedRaylibHandle,
        render_layer::ScreenSpaceRender,
    }};

use super::{Scenes, ScreenError};
use tracing::{debug, info, trace};

#[derive(Debug)]
pub struct HowToPlayScreen {
    is_btm_pressed: bool //Is back to menu button pressed
}

impl HowToPlayScreen {
    /// Construct a new `HowToPlayScreen`
    pub fn new() -> Self {
        Self {
            is_btm_pressed: false
        }
    }
}

impl Action<Scenes, ScreenError, GameContext> for HowToPlayScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, _context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running HowToPlayScreen for the first time");

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on HowToPlayScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        if self.is_btm_pressed {
            Ok(ActionFlag::SwitchState(Scenes::MainMenuScreen))
        }
        else{
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished HowToPlayScreen");
        self.is_btm_pressed = false;
        Ok(())
    }
}

impl ScreenSpaceRender for HowToPlayScreen {
    fn render_screen_space(
        &mut self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig
    ) {
        let screen_size = raylib.get_screen_size();

        // Render the background
        raylib.clear_background(Color::BLACK);
        raylib.draw_rectangle_lines(0, 0, screen_size.x as i32, screen_size.y as i32, config.colors.white);

        let screen_size = raylib.get_screen_size();

        //Mouse Position
        let mouse_position: Vector2 = raylib.get_mouse_position();

        let mouse_pressed: bool = raylib.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);

        raylib.draw_text(

            "How to Play",
            37,
            80,
            70,
            Color::BLUE,
        );

        raylib.draw_text(

            "How to Play",
            43,
            80,
            70,
            Color::RED,
        );

        raylib.draw_text(

            "How to Play",
            40,
            80,
            70,
            Color::WHITE,
        );

        //Back to Menu
        if Rectangle::new(35.0, screen_size.y as f32 - 80.0, 200.0, 40.0).check_collision_point_rec(mouse_position){
            raylib.draw_text(

                "BACK TO MENU",
                28,
                screen_size.y as i32 - 50,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "BACK TO MENU",
                22,
                screen_size.y as i32 - 50,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "BACK TO MENU",
                25,
                screen_size.y as i32 - 50,
                25,
                Color::WHITE,
            );

            if mouse_pressed{
                self.is_btm_pressed = true;
            }
        }
        else {
            raylib.draw_text(

                "BACK TO MENU",
                26,
                screen_size.y as i32 - 50,
                25,
                Color::RED,
            );
            raylib.draw_text(

                "BACK TO MENU",
                24,
                screen_size.y as i32 - 50,
                25,
                Color::BLUE,
            );
            raylib.draw_text(

                "BACK TO MENU",
                25,
                screen_size.y as i32 - 50,
                25,
                Color::WHITE,
            );
        }
    }
}
