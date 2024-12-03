mod engine;

use engine::renderer::Renderer;
use engine::input::InputHandler;
use engine::game_state::GameState;

use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;

fn main() {
    // Create an event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Platformer Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    // Initialize the renderer
    let renderer = block_on(Renderer::new(&window));

    // Initialize the input handler
    let mut input_handler = InputHandler::new();

    // Game state
    let mut game_state = GameState::new();

    // Timing variables
    let mut last_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Keep the event loop running

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    input_handler.handle_keyboard_input(input);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Calculate delta time
                let now = std::time::Instant::now();
                let delta_time = now.duration_since(last_frame_time).as_secs_f32();
                last_frame_time = now;

                // Update game state
                game_state.update(&input_handler, delta_time);

                // Render the frame
                game_state.render(&renderer);
                renderer.render();
            }
            _ => {}
        }
    });
}
