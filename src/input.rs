use std::collections::HashMap;

use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct Input {
    pressed: HashMap<VirtualKeyCode, bool>,
    prev_pressed: HashMap<VirtualKeyCode, bool>,
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Self {
            pressed: HashMap::new(),
            prev_pressed: HashMap::new(),
        }
    }

    pub fn key_pressed(&self, key: VirtualKeyCode) -> bool {
        *self.pressed.get(&key).unwrap_or(&false)
    }

    pub fn key_just_pressed(&self, key: VirtualKeyCode) -> bool {
        let prev_key = *self.prev_pressed.get(&key).unwrap_or(&false);
        
        !prev_key && self.key_pressed(key)
    }

    pub fn process_event(&mut self, input: KeyboardInput) {
        if input.virtual_keycode.is_some() {
            let keycode = input.virtual_keycode.unwrap();

            self.prev_pressed.insert(keycode, self.key_pressed(keycode));
            self.pressed.insert(keycode, input.state == ElementState::Pressed);
        }
    }

    pub fn update_states(&mut self) {
        for (key, value) in self.prev_pressed.clone().iter_mut() {
            *value = self.key_pressed(*key);
        }
    }
}