// game_state.rs
use crate::engine::input::InputHandler;
use crate::engine::renderer::Renderer;
use winit::event::VirtualKeyCode;
use std::collections::HashMap;

pub struct GameState {
    // Player position and velocity
    pub player_x: f32,
    pub player_y: f32,
    player_velocity_x: f32,
    player_velocity_y: f32,

    // Player state
    is_jumping: bool,
    is_crouching: bool,
    is_running: bool,

    // Constants
    player_speed: f32,
    gravity: f32,
    jump_force: f32,
    ground_y: f32,

    // Animation
    sprite_index: usize,
    frame_time: f32,
    current_action: String,
    actions: HashMap<String, (usize, usize)>,
}

impl GameState {
    pub fn new() -> Self {
        // Define the actions and their frame ranges
        let mut actions = HashMap::new();
        actions.insert("idle".to_string(), (0, 0));       // Idle: frame 0
        actions.insert("walk".to_string(), (1, 10));      // Walk: frames 1–10
        actions.insert("kick".to_string(), (11, 14));     // Kick: frames 11–14
        actions.insert("hurt".to_string(), (15, 17));     // Hurt: frames 15–17
        actions.insert("run".to_string(), (18, 24));      // Run: frames 18–24
        actions.insert("jump".to_string(), (6, 8));     // **Add this line**


        Self {
            player_x: 0.0,
            player_y: -0.5,
            player_velocity_x: 0.0,
            player_velocity_y: 0.0,
            is_jumping: false,
            is_crouching: false,
            is_running: false,
            player_speed: 1.0,
            gravity: -9.8,
            jump_force: 5.0,
            ground_y: -0.5,
            sprite_index: 0,
            frame_time: 0.0,
            current_action: "idle".to_string(),
            actions,
        }
    }

    pub fn update(&mut self, input_handler: &InputHandler, delta_time: f32) {
        // Reset horizontal velocity
        self.player_velocity_x = 0.0;

        // Movement input handling
        if input_handler.is_key_pressed(VirtualKeyCode::Left) {
            self.player_velocity_x -= self.player_speed;
        }
        if input_handler.is_key_pressed(VirtualKeyCode::Right) {
            self.player_velocity_x += self.player_speed;
        }

        // Jumping
        if input_handler.is_key_pressed(VirtualKeyCode::Up) && !self.is_jumping {
            self.player_velocity_y = self.jump_force;
            self.is_jumping = true;
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

        // Update current action based on movement
        if self.is_jumping {
            self.set_action("jump");
        } else if self.player_velocity_x.abs() > 0.0 {
            self.set_action("walk");
        } else {
            self.set_action("idle");
        }

        // Update animation frame
        self.update_animation(delta_time);
    }

    fn set_action(&mut self, action: &str) {
        if self.current_action != action {
            if let Some(&(start_frame, _)) = self.actions.get(action) {
                self.current_action = action.to_string();
                self.sprite_index = start_frame; // Reset to first frame of the new action
                self.frame_time = 0.0;
            } else {
                eprintln!("Action '{}' not found in actions HashMap", action);
                // Optionally, set to a default action or handle the error as needed
            }
        }
    }

    fn update_animation(&mut self, delta_time: f32) {
        let animation_speed = 0.1; // Change frame every 0.1 seconds
        self.frame_time += delta_time;

        if self.frame_time >= animation_speed {
            let (start_frame, end_frame) = self.actions[&self.current_action];
            self.sprite_index += 1;
            if self.sprite_index > end_frame {
                self.sprite_index = start_frame;
            }
            self.frame_time = 0.0;
        }
    }

    pub fn render(&self, renderer: &Renderer) {
        // Create the transform matrix
        let transform = Renderer::create_transform_matrix(
            self.player_x,
            self.player_y,
            0.1,
            0.1,
        );

        // Update the uniform buffer
        renderer.update_uniforms(transform, self.sprite_index as f32);
    }
}
