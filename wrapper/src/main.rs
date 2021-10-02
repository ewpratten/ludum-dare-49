use game::{GameConfig, StaticGameData, game_begin};

#[tokio::main]
async fn main() {
    // Enable logging
    tracing_subscriber::fmt::init();

    // Load the general config for the game
    // This happens here so we can properly track sentry events
    let mut game_config = GameConfig::load(
        StaticGameData::get("configs/application.json").expect("Failed to load application.json"),
    ).unwrap();

    // Connect to sentry
    let _sentry_guard = sentry::init((
        game_config.sentry_dsn.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            attach_stacktrace: true,
            ..Default::default()
        },
    ));

    // Start the game
    game_begin(&mut game_config).await.unwrap();
}
