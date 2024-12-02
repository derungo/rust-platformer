mod engine;

use engine::{game_loop::GameLoop, renderer::Renderer};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;

fn main() {
    // Create a window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Platformer Engine")
        .build(&event_loop)
        .unwrap();

    // Initialize the renderer
    let mut renderer = block_on(Renderer::new(&window));

    // Initialize the game loop
    let game_loop = GameLoop::new(60);

    // Run the game loop
    game_loop.run(|_delta_time| {
        renderer.render();
    });
}
