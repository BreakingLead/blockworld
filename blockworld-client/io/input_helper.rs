use std::collections::HashSet;

use glam::{vec2, Vec2};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::Key,
};

/// Tracker for the pressing keys
#[derive(Default, Debug)]
pub struct InputState {
    pub mouse_delta: Vec2,
    pub pressing_keys: HashSet<Key>,
}

impl InputState {
    pub fn is_key_pressing(&self, key: Key) -> bool {
        self.pressing_keys.contains(&key)
    }

    pub fn handle_device_event(&mut self, event: &winit::event::DeviceEvent) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                self.mouse_delta = vec2(delta.0 as f32, delta.1 as f32);
            }
            _ => (),
        }
    }

    pub fn handle_key_event(&mut self, event: &KeyEvent) {
        let key = &event.logical_key;
        match event.state {
            ElementState::Pressed => {
                self.pressing_keys.insert(key.clone());
            }
            ElementState::Released => {
                self.pressing_keys.remove(&key);
            }
        }
    }
}
