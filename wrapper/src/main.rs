use game::game_begin;

fn main() {
    // Enable logging
    tracing_subscriber::fmt::init();

    // Start the game
    game_begin();
}
