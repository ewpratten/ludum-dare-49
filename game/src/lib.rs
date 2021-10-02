#![feature(derive_default_enum)]
#![feature(custom_inner_attributes)]
#![feature(stmt_expr_attributes)]
#![feature(async_await)]
#![feature(c_variadic)]
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
#![clippy::msrv = "1.57.0"]

use std::{cell::RefCell, sync::mpsc::TryRecvError};

use discord_sdk::activity::ActivityBuilder;
use raylib::prelude::*;
use tracing::{error, info};
use utilities::discord::DiscordConfig;

use crate::{
    context::GameContext,
    discord_rpc::{maybe_set_discord_presence, try_connect_to_local_discord},
    scenes::{build_screen_state_machine, Scenes},
    utilities::{
        game_config::FinalShaderConfig,
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
#[macro_use]
extern crate approx;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate async_trait;

mod context;
mod discord_rpc;
mod scenes;
mod utilities;
pub use utilities::{datastore::StaticGameData, game_config::GameConfig};
mod character;

/// The game entrypoint
pub async fn game_begin(game_config: &mut GameConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Set up profiling
    #[cfg(debug_assertions)]
    let _puffin_server =
        puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap();
    puffin::set_scopes_on(true);

    // Attempt to connect to a locally running Discord instance for rich presence access
    let discord_config = DiscordConfig::load(
        StaticGameData::get("configs/discord.json").expect("Failed to load discord.json"),
    )
    .unwrap();
    let discord_rpc = match try_connect_to_local_discord(&discord_config).await {
        Ok(client) => Some(client),
        Err(err) => match err {
            utilities::discord::rpc::DiscordError::ConnectionTimeout(time) => {
                error!(
                    "Could not find or connect to a local Discord instance after {} seconds",
                    time
                );
                None
            }
            _ => panic!("Failed to connect to Discord: {}", err),
        },
    };
    maybe_set_discord_presence(
        &discord_rpc,
        ActivityBuilder::default().details("Game starting"),
    )
    .await
    .unwrap();

    // Build an MPSC for the game to send rich presence data to discord
    let (send_discord_rpc, recv_discord_rpc) = std::sync::mpsc::channel();

    let context;
    let raylib_thread;
    {
        // Set up FFI access to raylib
        // hook_raylib_logging();
        let (mut rl, thread) = raylib::init()
            .size(
                game_config.base_window_size.0,
                game_config.base_window_size.1,
            )
            .title(&format!("[{}]", game_config.name))
            .vsync()
            .msaa_4x()
            .resizable()
            .replace_logger()
            .build();
        rl.set_exit_key(None);
        raylib_thread = thread;

        // Build the game context
        context = Box::new(GameContext {
            renderer: RefCell::new(rl.into()),
            config: game_config.clone(),
            discord_rpc_send: send_discord_rpc,
        });
    }

    // Get the main state machine
    info!("Setting up the scene management state machine");
    let mut game_state_machine =
        build_screen_state_machine(&mut context.renderer.borrow_mut(), &raylib_thread).unwrap();
    game_state_machine
        .force_change_state(Scenes::LoadingScreen)
        .unwrap();

    // Create a dynamic texture to draw to for processing by shaders
    info!("Allocating a resizable texture for the screen");
    let mut dynamic_texture =
        DynScreenTexture::new(&mut context.renderer.borrow_mut(), &raylib_thread)?;

    // Load the pixel art shader
    info!("Loading the pixel art shader");
    let pixel_shader_config = FinalShaderConfig::load(
        StaticGameData::get("configs/final_shader.json").expect("Failed to load final_shader.json"),
    )
    .unwrap();
    let mut pixel_shader = ShaderWrapper::new(
        None,
        Some(StaticGameData::get("shaders/pixelart.fs")).expect("Failed to load pixelart.fs"),
        vec![
            "viewport",
            "pixelScale",
            "warpFactor",
            "scanlineDarkness",
            "bloomSamples",
            "bloomQuality",
        ],
        &mut context.renderer.borrow_mut(),
        &raylib_thread,
    )?;

    while !context.renderer.borrow().window_should_close() {
        // Profile the main game loop
        puffin::profile_scope!("main_loop");
        puffin::GlobalProfiler::lock().new_frame();

        // Update the GPU texture that we draw to. This handles screen resizing and some other stuff
        dynamic_texture
            .update(&mut context.renderer.borrow_mut(), &raylib_thread)
            .unwrap();

        // If in dev mode, allow a debug key
        #[cfg(debug_assertions)]
        {
            if context
                .renderer
                .borrow()
                .is_key_pressed(KeyboardKey::KEY_F3)
            {
                game_config.debug_view = !game_config.debug_view;
            }
        }

        // Handle fullscreen shortcut
        if context
            .renderer
            .borrow()
            .is_key_pressed(KeyboardKey::KEY_F11)
        {
            context.renderer.borrow_mut().toggle_fullscreen();
        }

        // Switch into draw mode the unsafe way (using unsafe code here to avoid borrow checker hell)
        #[allow(unsafe_code)]
        unsafe {
            raylib::ffi::BeginDrawing();
        }

        // Fetch the screen size once to work with in render code
        let screen_size = context.renderer.borrow().get_screen_size();

        // Update the pixel shader to correctly handle the screen size
        pixel_shader.set_variable("viewport", screen_size)?;
        pixel_shader.set_variable(
            "pixelScale",
            Vector2::new(
                pixel_shader_config.pixel_scale,
                pixel_shader_config.pixel_scale,
            ),
        )?;
        pixel_shader.set_variable("warpFactor", pixel_shader_config.warp_factor)?;
        pixel_shader.set_variable("scanlineDarkness", pixel_shader_config.scanline_darkness)?;
        pixel_shader.set_variable("bloomSamples", pixel_shader_config.bloom_samples)?;
        pixel_shader.set_variable("bloomQuality", pixel_shader_config.bloom_quality)?;

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

        // Try to update discord
        match recv_discord_rpc.try_recv() {
            Ok(activity) => {
                if let Some(activity) = activity {
                    if let Err(e) = maybe_set_discord_presence(&discord_rpc, activity).await {
                        error!("Failed to update discord presence: {:?}", e);
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                error!("Discord RPC channel disconnected");
                continue;
            }
        }
    }
    Ok(())
}
