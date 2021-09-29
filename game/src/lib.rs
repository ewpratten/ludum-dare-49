#![feature(derive_default_enum)]

use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

use discord_sdk::activity::ActivityBuilder;
use raylib::prelude::*;
use tracing::{error, info};
use utilities::{
    datastore::StaticGameData,
    discord::{DiscordConfig, DiscordRpcClient},
    game_config::GameConfig,
    math::rotate_vector,
};

use crate::{
    context::GameContext,
    scenes::build_screen_state_machine,
    utilities::{
        non_ref_raylib::HackedRaylibHandle,
        shaders::{
            shader::ShaderWrapper,
            util::{dynamic_screen_texture::DynScreenTexture, render_texture::render_to_texture},
        },
    },
};

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate serde;

mod context;
mod gfx;
mod scenes;
mod utilities;

/// The game entrypoint
pub async fn game_begin() {
    // Load the general config for the game
    let game_config = GameConfig::load(
        StaticGameData::get("configs/application.json").expect("Failed to load application.json"),
    )
    .expect("Could not load general game config data");

    // Set up profiling
    #[cfg(debug_assertions)]
    let _puffin_server =
        puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap();
    puffin::set_scopes_on(true);

    // Attempt to connect to a locally running Discord instance for rich presence access
    let discord_config = DiscordConfig::load(
        StaticGameData::get("configs/discord.json").expect("Failed to load discord.json"),
    )
    .expect("Could not load Discord config data");
    let discord_rpc =
        match DiscordRpcClient::new(discord_config.app_id, discord_sdk::Subscriptions::ACTIVITY)
            .await
        {
            Ok(client) => Some(client),
            Err(err) => {
                error!("Could not connect to or find a locally running Discord instance.");
                error!("Discord connection error: {:?}", err);
                None
            }
        };

    if let Some(rpc) = discord_rpc {
        rpc.set_rich_presence(ActivityBuilder::default().details("Testing..."))
            .await
            .unwrap();
    }

    // Get the main state machine
    let mut game_state_machine =
        build_screen_state_machine().expect("Could not init state main state machine");

    let mut context;
    let mut raylib_thread;
    {
        // Set up FFI access to raylib
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title(&game_config.name)
            .vsync()
            .msaa_4x()
            .resizable()
            .build();
        rl.set_exit_key(None);
        raylib_thread = thread;

        // Build the game context
        context = Box::new(GameContext::new(RefCell::new(rl.into())));
    }

    // Create a dynamic texture to draw to for processing by shaders
    info!("Allocating a resizable texture for the screen");
    let mut dynamic_texture =
        DynScreenTexture::new(&mut context.renderer.borrow_mut(), &raylib_thread)
            .expect("Failed to allocate a screen texture");

    // Load the pixel art shader
    info!("Loading the pixel art shader");
    let mut pixel_shader = ShaderWrapper::new(
        None,
        Some(StaticGameData::get("shaders/pixelart.fs")).expect("Failed to load pixelart.fs"),
        vec!["viewport"],
        &mut context.renderer.borrow_mut(),
        &raylib_thread,
    )
    .unwrap();

    info!("Starting the render loop");
    while !context.renderer.borrow().window_should_close() {
        // Profile the main game loop
        puffin::profile_scope!("main_loop");
        puffin::GlobalProfiler::lock().new_frame();

        // Update the GPU texture that we draw to. This handles screen resizing and some other stuff
        dynamic_texture
            .update(&mut context.renderer.borrow_mut(), &raylib_thread)
            .unwrap();

        // Switch into draw mode (using unsafe code here to avoid borrow checker hell)
        unsafe {
            raylib::ffi::BeginDrawing();
        }
        // let mut d = rl.begin_drawing(&thread);

        // Fetch the screen size once to work with in render code
        let screen_size = Vector2::new(
            context.renderer.borrow().get_screen_width() as f32,
            context.renderer.borrow().get_screen_height() as f32,
        );

        // Update the pixel shader to correctly handle the screen size
        pixel_shader.set_variable("viewport", screen_size).unwrap();

        // Render the game via the pixel shader
        render_to_texture(&mut dynamic_texture, || {
            // Profile the internal render code
            puffin::profile_scope!("internal_shaded_render");

            // Run a state machine iteration
            // let x = (context.renderer, context);
            let result = game_state_machine.run(&context);

            if let Err(err) = result {
                error!("Main state machine encountered an error while running!");
                error!("Main thread crash!!");
                error!("Cannot recover from error");
                panic!("{:?}", err);
            }
        });

        // Send the texture to the GPU to be drawn
        pixel_shader.process_texture_and_render(
            &mut context.renderer.borrow_mut(),
            &raylib_thread,
            &dynamic_texture,
        );

        // We MUST end draw mode
        unsafe {
            raylib::ffi::EndDrawing();
        }
    }
}
