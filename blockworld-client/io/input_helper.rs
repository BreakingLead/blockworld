use std::collections::HashSet;

use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{Key, NamedKey},
};

/// Tracker for the pressing keys
#[derive(Default)]
pub struct InputState {
    pressing_keys: HashSet<Key>,
}

impl InputState {
    pub fn is_key_pressing(&self, key: Key) -> bool {
        self.pressing_keys.contains(&key)
    }

    pub fn handle_event<'a>(&mut self, event: &'a KeyEvent) {
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
