use game::game_begin;

#[tokio::main]
async fn main() {
    // Enable logging
    tracing_subscriber::fmt::init();

    // Start the game
    game_begin().await.unwrap();
}
