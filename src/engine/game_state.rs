use crate::engine::input::InputHandler;
use crate::engine::renderer::Renderer;
use crate::engine::constants::{SPRITE_WIDTH, SPRITE_HEIGHT, GROUND_LEVEL, PLAYER_SPEED, GRAVITY, JUMP_FORCE, ANIMATION_SPEED};
use winit::event::VirtualKeyCode;
use std::collections::HashMap;

/// Represents the state of the game, including the player's position,
/// actions, and physics-related properties.
pub struct GameState {
    /// Player's horizontal position in the game world.
    pub player_x: f32,
    /// Player's vertical position in the game world.
    pub player_y: f32,
    player_velocity_x: f32,
    player_velocity_y: f32,

    // Player state
    is_jumping: bool,
    is_crouching: bool,
    is_running: bool,
    is_kicking: bool,
    pub facing_right: bool,

    // Animation
    pub sprite_index: usize,
    frame_time: f32,
    current_action: String,
    actions: HashMap<String, (usize, usize)>,
}

impl GameState {
    /// Creates a new `GameState` instance with default values.
    pub fn new() -> Self {
        let mut actions = HashMap::new();
        actions.insert("idle".to_string(), (0, 0));
        actions.insert("walk".to_string(), (1, 10));
        actions.insert("kick".to_string(), (11, 13));
        actions.insert("hurt".to_string(), (14, 16));
        actions.insert("run".to_string(), (17, 23));
        actions.insert("jump".to_string(), (6, 8));
        actions.insert("crouch_walk".to_string(), (19, 23));
        actions.insert("crouch_idle".to_string(), (18, 18));

        Self {
            player_x: 0.0,
            player_y: GROUND_LEVEL + (SPRITE_HEIGHT / 2.0),
            player_velocity_x: 0.0,
            player_velocity_y: 0.0,
            is_jumping: false,
            is_crouching: false,
            is_running: false,
            is_kicking: false,
            facing_right: true,
            sprite_index: 0,
            frame_time: 0.0,
            current_action: "idle".to_string(),
            actions,
        }
    }

    /// Updates the game state, including handling player input,
    /// physics (gravity), and animations.
    ///
    /// # Arguments
    ///
    /// * `input_handler` - Provides the state of input keys.
    /// * `delta_time` - Time elapsed since the last frame.
    pub fn update(&mut self, input_handler: &InputHandler, delta_time: f32) {
        self.player_velocity_x = 0.0;

        // Handle running
        self.is_running = input_handler.is_key_pressed(VirtualKeyCode::LShift);

        // Handle horizontal movement
        let mut is_moving = false;
        if input_handler.is_key_pressed(VirtualKeyCode::A) {
            self.player_velocity_x -= if self.is_running { PLAYER_SPEED * 1.5 } else { PLAYER_SPEED };
            self.facing_right = false;
            is_moving = true;
        }
        if input_handler.is_key_pressed(VirtualKeyCode::D) {
            self.player_velocity_x += if self.is_running { PLAYER_SPEED * 1.5 } else { PLAYER_SPEED };
            self.facing_right = true;
            is_moving = true;
        }

        // Handle crouching
        self.is_crouching = input_handler.is_key_pressed(VirtualKeyCode::LControl);

        // Handle kicking
        self.is_kicking = input_handler.is_key_pressed(VirtualKeyCode::E);

        // Handle jumping
        if input_handler.is_key_pressed(VirtualKeyCode::Space) && !self.is_jumping && !self.is_crouching {
            self.player_velocity_y = JUMP_FORCE;
            self.is_jumping = true;
        }

        // Apply gravity
        self.player_velocity_y += GRAVITY * delta_time;

        // Update position
        self.player_x += self.player_velocity_x * delta_time;
        self.player_y += self.player_velocity_y * delta_time;

        // Ground collision
        let player_bottom = self.player_y - (SPRITE_HEIGHT / 2.0);
        if player_bottom <= GROUND_LEVEL {
            self.player_y = GROUND_LEVEL + (SPRITE_HEIGHT / 2.0);
            self.player_velocity_y = 0.0;
            self.is_jumping = false;
        }

        // Update action
        self.update_action(is_moving);

        // Update animation frame
        self.update_animation(delta_time);
    }

    /// Updates the player's current action based on their state and movement.
    ///
    /// # Arguments
    ///
    /// * `is_moving` - Whether the player is currently moving.
    fn update_action(&mut self, is_moving: bool) {
        if self.is_kicking {
            self.set_action("kick");
        } else if self.is_jumping {
            self.set_action("jump");
        } else if self.is_crouching {
            if is_moving {
                self.set_action("crouch_walk");
            } else {
                self.set_action("crouch_idle");
            }
        } else if is_moving {
            if self.is_running {
                self.set_action("run");
            } else {
                self.set_action("walk");
            }
        } else {
            self.set_action("idle");
        }
    }

    /// Sets the current action and resets the animation frame to the start of the action.
    ///
    /// # Arguments
    ///
    /// * `action` - The name of the action to set.
    fn set_action(&mut self, action: &str) {
        if self.current_action != action {
            if let Some(&(start_frame, _)) = self.actions.get(action) {
                self.current_action = action.to_string();
                self.sprite_index = start_frame;
                self.frame_time = 0.0;
            } else {
                eprintln!("Action '{}' not found in actions HashMap", action);
            }
        }
    }

    /// Updates the animation frame based on the elapsed time and current action.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - Time elapsed since the last frame.
    fn update_animation(&mut self, delta_time: f32) {
        self.frame_time += delta_time;

        if self.frame_time >= ANIMATION_SPEED {
            let (start_frame, end_frame) = self.actions[&self.current_action];

            if start_frame == end_frame {
                self.sprite_index = start_frame;
            } else {
                self.sprite_index += 1;
                if self.sprite_index > end_frame {
                    if self.current_action == "kick" {
                        self.is_kicking = false;
                        self.set_action("idle");
                    } else {
                        self.sprite_index = start_frame;
                    }
                }
            }

            self.frame_time = 0.0;
        }
    }
}
