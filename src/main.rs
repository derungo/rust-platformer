mod engine;

use engine::renderer::Renderer;
use engine::input::InputHandler;

use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;

struct GameState {
    player_x: f32,
    player_y: f32,
    player_velocity_x: f32,
    player_velocity_y: f32,
    player_speed: f32,
    is_jumping: bool,
    is_crouching: bool,
    gravity: f32,
    jump_force: f32,
    ground_y: f32,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_x: 0.0,
            player_y: -0.5, // Start on the ground
            player_velocity_x: 0.0,
            player_velocity_y: 0.0,
            player_speed: 1.0, // Adjusted for coordinate system
            is_jumping: false,
            is_crouching: false,
            gravity: -9.8,
            jump_force: 5.0,
            ground_y: -0.5, // Y position of the ground
        }
    }

    fn update(&mut self, input_handler: &InputHandler, delta_time: f32) {
        // Horizontal movement
        if input_handler.is_key_pressed(VirtualKeyCode::Left)
            || input_handler.is_key_pressed(VirtualKeyCode::A)
        {
            self.player_velocity_x = -self.player_speed;
        } else if input_handler.is_key_pressed(VirtualKeyCode::Right)
            || input_handler.is_key_pressed(VirtualKeyCode::D)
        {
            self.player_velocity_x = self.player_speed;
        } else {
            self.player_velocity_x = 0.0;
        }

        // Jumping
        if (input_handler.is_key_pressed(VirtualKeyCode::Up)
            || input_handler.is_key_pressed(VirtualKeyCode::W))
            && !self.is_jumping
        {
            self.player_velocity_y = self.jump_force;
            self.is_jumping = true;
        }

        // Crouching
        if input_handler.is_key_pressed(VirtualKeyCode::Down)
            || input_handler.is_key_pressed(VirtualKeyCode::S)
        {
            self.is_crouching = true;
        } else {
            self.is_crouching = false;
        }

        // Apply gravity
        self.player_velocity_y += self.gravity * delta_time;

        // Update positions
        self.player_x += self.player_velocity_x * delta_time;
        self.player_y += self.player_velocity_y * delta_time;

        // Ground collision
        if self.player_y <= self.ground_y {
            self.player_y = self.ground_y;
            self.player_velocity_y = 0.0;
            self.is_jumping = false;
        }

        // Clamp positions
        self.player_x = self.player_x.clamp(-1.0, 1.0);
    }

    fn render(&self, renderer: &Renderer) {
        // Render the player
        let player_height = if self.is_crouching { 0.05 } else { 0.1 };
        renderer.update_transform_matrix(self.player_x, self.player_y, 0.1, player_height);

        // Render the ground
        renderer.render_ground();
    }
}

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
