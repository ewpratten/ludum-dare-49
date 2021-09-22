use discord_sdk::activity::ActivityBuilder;
use raylib::prelude::*;
use shaders::{
    shader::ShaderWrapper,
    util::{dynamic_screen_texture::DynScreenTexture, render_texture::render_to_texture},
};
use tracing::{error, info};
use utilities::{
    datastore::StaticGameData,
    discord::{DiscordConfig, DiscordRpcClient},
    game_config::GameConfig,
    math::rotate_vector,
};

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate serde;

mod shaders;
mod utilities;

/// The game entrypoint
pub async fn game_begin() {
    // Load the general config for the game
    let game_config = GameConfig::load(
        StaticGameData::get("configs/application.json").expect("Failed to load application.json"),
    )
    .expect("Could not load general game config data");

    // Set up profiling
    // #[cfg(debug_assertions)]
    // {
    let _puffin_server =
        puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap();
    puffin::set_scopes_on(true);
    // }

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

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title(&game_config.name)
        // .vsync()
        .msaa_4x()
        .resizable()
        .build();

    // Create a dynamic texture to draw to for processing by shaders
    info!("Allocating a resizable texture for the screen");
    let mut dynamic_texture =
        DynScreenTexture::new(&mut rl, &thread).expect("Failed to allocate a screen texture");

    // Load the pixel art shader
    info!("Loading the pixel art shader");
    let mut pixel_shader = ShaderWrapper::new(
        None,
        Some(StaticGameData::get("shaders/pixelart.fs")).expect("Failed to load pixelart.fs"),
        vec!["viewport"],
        &mut rl,
        &thread,
    )
    .unwrap();

    info!("Starting the render loop");
    while !rl.window_should_close() {
        puffin::profile_scope!("main_loop");
        puffin::GlobalProfiler::lock().new_frame();
        dynamic_texture.update(&mut rl, &thread).unwrap();
        let mut d = rl.begin_drawing(&thread);
        let screen_size = Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32);

        pixel_shader.set_variable("viewport", screen_size).unwrap();

        render_to_texture(&mut dynamic_texture, || {
            puffin::profile_scope!("internal_shaded_render");
            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

            let angle = (d.get_time() as f32 * 80.0).to_radians();
            let screen_center = Vector2::new(
                d.get_screen_width() as f32 / 2.0,
                d.get_screen_height() as f32 / 2.0,
            );
            let top = rotate_vector(Vector2::new(0.0, -100.0), angle) + screen_center;
            let right = rotate_vector(Vector2::new(100.0, 0.0), angle) + screen_center;
            let left = rotate_vector(Vector2::new(-100.0, 0.0), angle) + screen_center;

            d.draw_triangle(top, left, right, Color::BLACK);
            d.draw_fps(10, 100);
        });

        pixel_shader.process_texture_and_render(&mut d, &thread, &dynamic_texture);
    }
}
