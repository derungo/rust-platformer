// game_loop.rs
use crate::engine::{GameState, InputHandler, Renderer};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;


pub fn run() {
    // Create an event loop and a window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Platformer Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .expect("Failed to create window.");

    // Initialize the renderer
    let renderer = block_on(Renderer::new(&window));

    // Initialize the input handler
    let mut input_handler = InputHandler::new();

    // Initialize the game state
    let mut game_state = GameState::new();

    // Timing variables for frame timing
    let mut last_frame_time = std::time::Instant::now();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Keep the event loop running

        match event {
            // Handle window events
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit; // Exit the application
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    input_handler.handle_keyboard_input(input); // Update input handler
                }
                _ => {}
            },
            // Handle main events cleared
            Event::MainEventsCleared => {
                // Calculate delta time
                let now = std::time::Instant::now();
                let delta_time = now.duration_since(last_frame_time).as_secs_f32();
                last_frame_time = now;

                // Update game state (logic and animations)
                game_state.update(&input_handler, delta_time);

                // Render the current frame
                game_state.render(&renderer);

                // Execute the rendering pipeline
                renderer.render();

                // Frame limiting for consistent rendering (60 FPS)
                let frame_duration = std::time::Duration::from_secs_f32(1.0 / 60.0);
                std::thread::sleep(frame_duration.saturating_sub(now.elapsed()));
            }
            _ => {}
        }
    });
}
