mod game_loop;
mod engine;

fn main() {
    // Initialize the logger
    env_logger::init();

    // Log that the game loop is starting
    log::info!("Starting the game loop...");
    game_loop::run();
}
