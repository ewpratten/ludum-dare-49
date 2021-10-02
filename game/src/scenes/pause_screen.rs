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
pub struct PauseScreen {}

impl PauseScreen {
    /// Construct a new `PauseScreen`
    pub fn new() -> Self {
        Self {}
    }
}

impl Action<Scenes, ScreenError, GameContext> for PauseScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running PauseScreen for the first time");

        // Update discord
        if let Err(e) = context.discord_rpc_send.send(Some(
            ActivityBuilder::default().details("paused").assets(
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
        trace!("execute() called on PauseScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);
        //Screen Size
        let screen_size = context.renderer.borrow_mut().get_screen_size();


        let centered_x_menu = (screen_size.x as f32 / 2.0) - 120.0;
        let centered_y_menu = (screen_size.y as f32 / 2.0) + 100.0;
        let centered_x_paused = (screen_size.x as f32 / 2.0) - 220.0;
        let centered_y_paused = (screen_size.y as f32 / 2.0) - 40.0;


        //Mouse Position
        let mouse_position: Vector2 = context.renderer.borrow_mut().get_mouse_position();
        //Mouse Input
        let is_left_click = context
            .renderer
            .borrow_mut()
            .is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

        //"Hitboxes" for the resume and Main menu buttons

        //For Paused
        if is_left_click
            && Rectangle::new(centered_x_paused, centered_y_paused, 435.0, 80.0).check_collision_point_rec(mouse_position)
        {
            return Ok(ActionFlag::SwitchState(Scenes::InGameScene));
        }
        //For Menu
        if is_left_click
            && Rectangle::new(centered_x_menu, centered_y_menu, 200.0, 50.0).check_collision_point_rec(mouse_position)
        {
            return Ok(ActionFlag::SwitchState(Scenes::MainMenuScreen));
        }

        if context
            .renderer
            .borrow_mut()
            .is_key_pressed(KeyboardKey::KEY_ESCAPE)
        {
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished PauseScreen");
        Ok(())
    }
}

impl ScreenSpaceRender for PauseScreen {
    fn render_screen_space(
        &self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig,
    ) {
        let screen_size = raylib.get_screen_size();

        // Render the background
        raylib.clear_background(Color::BLACK.fade(50.0));

        //Mouse Position
        let mouse_position: Vector2 = raylib.get_mouse_position();
        //Mouse Input
        let is_left_click = raylib.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

        raylib.draw_rectangle_lines(0, 0, screen_size.x as i32, screen_size.y as i32, config.colors.white);

        //Pause Menu Texts With Glitchy Effect
        raylib.draw_text(
            "Paused",
            (screen_size.x as i32 / 2) - 223,
            (screen_size.y as i32 / 2) - 40,
            120,
            Color::RED,
        );
        raylib.draw_text(
            "Paused",
            (screen_size.x as i32 / 2) - 217,
            (screen_size.y as i32 / 2) - 40,
            120,
            Color::BLUE,
        );
        raylib.draw_text(
            "Paused",
            (screen_size.x as i32 / 2) - 220,
            (screen_size.y as i32 / 2) - 40,
            120,
            Color::WHITE,
        );
        raylib.draw_text(
            "Click To Resume",
            (screen_size.x as i32 / 2) - 80,
            (screen_size.y as i32 / 2) + 60,
            20,
            Color::RED,
        );
        raylib.draw_text(
            "Click To Resume",
            (screen_size.x as i32 / 2) - 80,
            (screen_size.y as i32 / 2) + 60,
            20,
            Color::BLUE,
        );
        raylib.draw_text(
            "Click To Resume",
            (screen_size.x as i32 / 2) - 80,
            (screen_size.y as i32 / 2) + 60,
            20,
            Color::WHITE,
        );
        raylib.draw_text(
            "Main Menu",
            (screen_size.x as i32 / 2) - 123,
            (screen_size.y as i32 / 2) + 100,
            50,
            Color::RED,
        );
        raylib.draw_text(
            "Main Menu",
            (screen_size.x as i32 / 2) - 117,
            (screen_size.y as i32 / 2) + 100,
            50,
            Color::BLUE,
        );
        raylib.draw_text(
            "Main Menu",
            (screen_size.x as i32 / 2) - 120,
            (screen_size.y as i32 / 2) + 100,
            50,
            Color::WHITE,
        );

        //Push comment

        //Variables for centering
        let centered_x_menu = (screen_size.x as f32 / 2.0) - 120.0;
        let centered_y_menu = (screen_size.y as f32 / 2.0) + 100.0;
        let centered_x_paused = (screen_size.x as f32 / 2.0) - 220.0;
        let centered_y_paused = (screen_size.y as f32 / 2.0) - 40.0;


        if Rectangle::new(centered_x_menu, centered_y_menu, 200.0, 50.0).check_collision_point_rec(mouse_position) {
            raylib.draw_text(
                "Main Menu",
                (screen_size.x as i32 / 2) - 116,
                (screen_size.y as i32 / 2) + 100,
                50,
                Color::RED,
            );
            raylib.draw_text(
                "Main Menu",
                (screen_size.x as i32 / 2) - 124,
                (screen_size.y as i32 / 2) + 100,
                50,
                Color::BLUE,
            );
            raylib.draw_text(
                "Main Menu",
                (screen_size.x as i32 / 2) - 120,
                (screen_size.y as i32 / 2) + 100,
                50,
                Color::WHITE,
            );

        }

        if Rectangle::new(centered_x_paused, centered_y_paused, 435.0, 80.0).check_collision_point_rec(mouse_position) {
            raylib.draw_text(
                "Paused",
                (screen_size.x as i32 / 2) - 215,
                (screen_size.y as i32 / 2) - 40,
                120,
                Color::BLUE,
            );
            raylib.draw_text(
                "Paused",
                (screen_size.x as i32 / 2) - 225,
                (screen_size.y as i32 / 2) - 40,
                120,
                Color::RED,
            );
            raylib.draw_text(
                "Paused",
                (screen_size.x as i32 / 2) - 220,
                (screen_size.y as i32 / 2) - 40,
                120,
                Color::WHITE,
            );
        }
    }
}
