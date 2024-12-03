use crate::engine::renderer::{vertex::Vertex, Renderer};
use crate::engine::input::InputHandler;
use winit::event::VirtualKeyCode;
use std::collections::HashMap;

pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    player_velocity_x: f32,
    player_velocity_y: f32,
    player_speed: f32,
    is_jumping: bool,
    is_crouching: bool,
    gravity: f32,
    jump_force: f32,
    ground_y: f32,
    sprite_index: usize,
    frame_time: f32,
    current_action: String, // Current action name
    actions: HashMap<String, (usize, usize)>, // Action name -> (start_frame, end_frame)
}

impl GameState {
    pub fn new() -> Self {
        let mut actions = HashMap::new();
        actions.insert("idle".to_string(), (0, 0));        // Idle: frame 0
        actions.insert("walk".to_string(), (0, 10));      // Walk: frames 0–10
        actions.insert("kick".to_string(), (11, 14));     // Kick: frames 11–14
        actions.insert("hurt".to_string(), (15, 17));     // Hurt: frames 15–17
        actions.insert("run".to_string(), (18, 24));      // Run: frames 18–24

        Self {
            player_x: 0.0,
            player_y: -0.5,
            player_velocity_x: 0.0,
            player_velocity_y: 0.0,
            player_speed: 1.0,
            is_jumping: false,
            is_crouching: false,
            gravity: -9.8,
            jump_force: 5.0,
            ground_y: -0.5,
            sprite_index: 0,
            frame_time: 0.0,
            current_action: "idle".to_string(),
            actions,
        }
    }

    pub fn set_action(&mut self, action: &str) {
        if self.current_action != action {
            self.current_action = action.to_string();
            self.sprite_index = self.actions[&self.current_action].0; // Reset to first frame of the new action
        }
    }

    pub fn update(&mut self, input_handler: &InputHandler, delta_time: f32) {
        // Determine the current action based on input
        if input_handler.is_key_pressed(VirtualKeyCode::Left) {
            self.player_velocity_x = -self.player_speed;
            self.set_action("walk"); // Set to walking
        } else if input_handler.is_key_pressed(VirtualKeyCode::Right) {
            self.player_velocity_x = self.player_speed;
            self.set_action("walk"); // Set to walking
        } else {
            self.player_velocity_x = 0.0;
            self.set_action("idle"); // Default to idle
        }
    
        // Jumping logic
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
    
        // Clamp horizontal position
        self.player_x = self.player_x.clamp(-1.0, 1.0);
    
        // Animation timing
        let animation_speed = 0.1; // Change sprite every 0.1 seconds
        self.frame_time += delta_time;
    
        if self.frame_time >= animation_speed {
            let (start_frame, end_frame) = self.actions[&self.current_action];
            self.sprite_index = if self.sprite_index >= end_frame {
                start_frame
            } else {
                self.sprite_index + 1
            };
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
        renderer.update_uniforms(&renderer.queue, transform, self.sprite_index as f32);
    
        // Execute the rendering pipeline
        renderer.render();
    }
    }
    
    

