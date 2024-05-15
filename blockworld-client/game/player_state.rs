use std::collections::HashSet;

use winit::keyboard::{Key, KeyCode, NamedKey};

use crate::{io::input_helper::InputState, render::draw::State};

// Best not to pub here
// I'd like to change it later
#[derive(Default)]
pub struct PlayerState {
    pub forward: bool,
    pub backward: bool,
    pub ascend: bool,
    pub descend: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerState {
    pub fn update(&mut self, state: &InputState) {
        self.ascend = false;
        self.descend = false;
        self.left = false;
        self.right = false;
        self.forward = false;
        self.backward = false;
        if state.is_key_pressing(Key::Character("w".into())) {
            self.forward = true;
        }
        if state.is_key_pressing(Key::Character("a".into())) {
            self.left = true;
        }
        if state.is_key_pressing(Key::Character("s".into())) {
            self.backward = true;
        }
        if state.is_key_pressing(Key::Character("d".into())) {
            self.right = true;
        }
        if state.is_key_pressing(Key::Named(NamedKey::Space)) {
            self.ascend = true;
        }
        if state.is_key_pressing(Key::Named(NamedKey::Shift)) {
            self.descend = true;
        }
    }
}
