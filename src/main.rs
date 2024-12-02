mod engine;

use engine::renderer::Renderer;
use engine::input::InputHandler;

use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;
use std::time::{Duration, Instant};

struct GameState {
    player_x: f32,
    player_y: f32,
    player_velocity: f32,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_x: 0.0,
            player_y: 0.0,
            player_velocity: 200.0,
        }
    }

    fn update(&mut self, input_handler: &InputHandler, delta_time: f32) {
        if input_handler.is_key_pressed(VirtualKeyCode::Left) || input_handler.is_key_pressed(VirtualKeyCode::A) {
            self.player_x -= self.player_velocity * delta_time;
        }

        if input_handler.is_key_pressed(VirtualKeyCode::Right) || input_handler.is_key_pressed(VirtualKeyCode::D) {
            self.player_x += self.player_velocity * delta_time;
        }

        // Ensure the rectangle stays within the window bounds
        self.player_x = self.player_x.clamp(-1.0, 1.0);
    }

    fn render(&self, renderer: &Renderer) {
        // Translate normalized coordinates to match screen space for simplicity
        renderer.render_rectangle(self.player_x, self.player_y, 0.1, 0.1); // Rectangle size is 0.1 in normalized units
    }
}

fn main() {
    // Create a window
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
                }
                _ => {}
            },
            // Main game loop logic
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = (now - last_update_time).as_secs_f32();

                // Update game state if enough time has passed
                if delta_time >= target_frame_duration.as_secs_f32() {
                    last_update_time = now;

                    // Update the game state
                    game_state.update(&input_handler, delta_time);
                }

                // Request a redraw
                window.request_redraw();
            },
            // Render the frame
            Event::RedrawRequested(_) => {
                game_state.render(&renderer);
            }
            _ => {}
        }
    });
}
