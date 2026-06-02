//! Keyboard input tracking.
//!
//! Maintains a set of currently-pressed keys and converts them
//! into a `MovementRecord` each frame for camera movement.

use std::collections::HashSet;

use glam::{vec2, Vec2};
use once_cell::sync::Lazy;
use winit::{
    event::{DeviceEvent, ElementState, KeyEvent},
    keyboard::{Key, NamedKey},
};

/// Which movement directions are currently active.
#[derive(Default)]
pub struct MovementRecord {
    pub forward: bool,
    pub backward: bool,
    pub ascend: bool,
    pub descend: bool,
    pub left: bool,
    pub right: bool,
}

/// Tracks currently-held keys via a set of `Key` values.
///
/// `window_init` calls `handle_key_event()` on each keyboard event;
/// `Camera::update()` calls `to_key_record()` to read movement state.
#[derive(Default, Debug)]
pub struct InputManager {
    pressing_keys: HashSet<Key>,
}

impl InputManager {
    /// Convert the current key state into a `MovementRecord` for camera movement.
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

    /// Register a key press or release.
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
