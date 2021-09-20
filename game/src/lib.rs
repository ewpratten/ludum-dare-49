use discord_sdk::activity::ActivityBuilder;
use tracing::error;
use utilities::{
    datastore::StaticGameData,
    discord::{DiscordConfig, DiscordRpcClient},
};
use raylib::prelude::*;

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate serde;

mod utilities;

/// The game entrypoint
pub async fn game_begin() {
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
        rpc.set_rich_presence(ActivityBuilder::default().details("Testing...")).await.unwrap();
    }

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
