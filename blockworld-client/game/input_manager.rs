use std::collections::HashSet;

use glam::{vec2, Vec2};
use once_cell::sync::Lazy;
use winit::{
    event::{DeviceEvent, ElementState, KeyEvent},
    keyboard::{Key, NamedKey},
};

use super::key_record::MovementRecord;

pub static mut GLOBAL_INPUT_MANAGER: Lazy<InputManager> = Lazy::new(|| InputManager::default());

/// Tracker for the pressing keys
#[derive(Default, Debug)]
pub struct InputManager {
    pressing_keys: HashSet<Key>,
}

impl InputManager {
    pub fn to_key_record(&self) -> MovementRecord {
        let mut s = MovementRecord::default();
        if self.is_key_pressing(Key::Character("w".into())) {
            s.forward = true;
        }
        if self.is_key_pressing(Key::Character("a".into())) {
            s.left = true;
        }
        if self.is_key_pressing(Key::Character("s".into())) {
            s.backward = true;
        }
        if self.is_key_pressing(Key::Character("d".into())) {
            s.right = true;
        }
        if self.is_key_pressing(Key::Named(NamedKey::Space)) {
            s.ascend = true;
        }
        if self.is_key_pressing(Key::Named(NamedKey::Shift)) {
            s.descend = true;
        }
        s
    }

    pub fn is_key_pressing(&self, key: Key) -> bool {
        self.pressing_keys.contains(&key)
    }

    pub fn handle_key_event(&mut self, event: &KeyEvent) {
        let key = &event.logical_key;
        match event.state {
            ElementState::Pressed => {
                self.pressing_keys.insert(key.clone());
            }
            ElementState::Released => {
                self.pressing_keys.remove(key);
            }
        }
    }

    pub fn handle_mouse_event(&mut self, event: &DeviceEvent) {}
}
