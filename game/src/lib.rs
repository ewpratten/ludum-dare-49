#![feature(derive_default_enum)]
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]

use std::cell::RefCell;

use discord_sdk::activity::ActivityBuilder;
use raylib::prelude::*;
use tracing::{error, info};
use utilities::{
    datastore::StaticGameData,
    discord::{DiscordConfig, DiscordRpcClient},
    game_config::GameConfig,
};

use crate::{
    context::GameContext,
    scenes::build_screen_state_machine,
    utilities::shaders::{
        shader::ShaderWrapper,
        util::{dynamic_screen_texture::DynScreenTexture, render_texture::render_to_texture},
    },
};

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate serde;

mod context;
mod scenes;
mod utilities;

/// The game entrypoint
pub async fn game_begin() -> Result<(), Box<dyn std::error::Error>> {
    // Load the general config for the game
    let game_config = GameConfig::load(
        StaticGameData::get("configs/application.json").expect("Failed to load application.json"),
    )?;

    // Set up profiling
    #[cfg(debug_assertions)]
    let _puffin_server =
        puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT))?;
    puffin::set_scopes_on(true);

    // Attempt to connect to a locally running Discord instance for rich presence access
    let discord_config = DiscordConfig::load(
        StaticGameData::get("configs/discord.json").expect("Failed to load discord.json"),
    )?;
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
            .await?;
    }

    // Get the main state machine
    let mut game_state_machine = build_screen_state_machine()?;

    let context;
    let raylib_thread;
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
        DynScreenTexture::new(&mut context.renderer.borrow_mut(), &raylib_thread)?;

    // Load the pixel art shader
    info!("Loading the pixel art shader");
    let mut pixel_shader = ShaderWrapper::new(
        None,
        Some(StaticGameData::get("shaders/pixelart.fs")).expect("Failed to load pixelart.fs"),
        vec!["viewport"],
        &mut context.renderer.borrow_mut(),
        &raylib_thread,
    )?;

    info!("Starting the render loop");
    while !context.renderer.borrow().window_should_close() {
        // Profile the main game loop
        puffin::profile_scope!("main_loop");
        puffin::GlobalProfiler::lock().new_frame();

        // Update the GPU texture that we draw to. This handles screen resizing and some other stuff
        dynamic_texture.update(&mut context.renderer.borrow_mut(), &raylib_thread)?;

        // Switch into draw mode the unsafe way (using unsafe code here to avoid borrow checker hell)
        #[allow(unsafe_code)]
        unsafe {
            raylib::ffi::BeginDrawing();
        }

        // Fetch the screen size once to work with in render code
        let screen_size = Vector2::new(
            context.renderer.borrow().get_screen_width() as f32,
            context.renderer.borrow().get_screen_height() as f32,
        );

        // Update the pixel shader to correctly handle the screen size
        pixel_shader.set_variable("viewport", screen_size)?;

        // Render the game via the pixel shader
        render_to_texture(&mut dynamic_texture, || {
            // Profile the internal render code
            puffin::profile_scope!("internal_shaded_render");

            // Run a state machine iteration
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
        #[allow(unsafe_code)]
        unsafe {
            raylib::ffi::EndDrawing();
        }
    }
    Ok(())
}
