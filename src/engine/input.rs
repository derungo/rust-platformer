use std::collections::HashSet;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct InputHandler {
    keys_pressed: HashSet<VirtualKeyCode>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
        }
    }

    pub fn update(&mut self, input: &KeyboardInput) {
        if let Some(key) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => {
                    self.keys_pressed.insert(key);
                }
                ElementState::Released => {
                    self.keys_pressed.remove(&key);
                }
            }
        }
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }
}
