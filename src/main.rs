mod engine;

use engine::{renderer::Renderer, input::InputHandler};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;
use std::time::{Duration, Instant};

fn main() {
    // Create a window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Platformer Engine")
        .build(&event_loop)
        .unwrap();

    // Initialize the renderer
    let mut renderer = block_on(Renderer::new(&window));

    // Initialize the input handler
    let mut input_handler = InputHandler::new();

    // Game loop timing
    let target_frame_duration = Duration::from_secs_f64(1.0 / 60.0); // 60 FPS
    let mut last_update_time = Instant::now();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            // Handle window events
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    input_handler.update(&input);
                    
                    // Handle movement keys
                    if input_handler.is_key_pressed(VirtualKeyCode::Left) || input_handler.is_key_pressed(VirtualKeyCode::A) {
                        println!("Player moving left!");
                    }
                    if input_handler.is_key_pressed(VirtualKeyCode::Right) || input_handler.is_key_pressed(VirtualKeyCode::D) {
                        println!("Player moving right!");
                    }
                    if input_handler.is_key_pressed(VirtualKeyCode::Space) {
                        println!("Player jumping!");
                    }
                    if input_handler.is_key_pressed(VirtualKeyCode::Escape) {
                        println!("I'm Melting!");
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            // Main game loop logic
            Event::MainEventsCleared => {
                // Calculate delta time
                let now = Instant::now();
                let delta_time = now - last_update_time;

                // Update game state if enough time has passed
                if delta_time >= target_frame_duration {
                    last_update_time = now;

                    // Update your game state here
                }

                // Request a redraw
                window.request_redraw();
            },
            // Render the frame
            Event::RedrawRequested(_) => {
                renderer.render();
            }
            _ => {}
        }
    });
}
