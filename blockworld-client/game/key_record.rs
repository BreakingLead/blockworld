use winit::keyboard::{Key, NamedKey};

use super::input_manager::InputManager;

// auto generate the immutable struct and getters
macro_rules! record {
    (
        $n:ident {
            $($property_name:ident: $property_type:ty),+
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $n {
            pub(super) $($property_name: $property_type),+
        }

        impl $n {
            $(pub fn $property_name(&self) -> $property_type {
                self.$property_name
            })+
        }
    };
}

record! {
    MovementRecord {
        forward: bool,
        backward: bool,
        ascend: bool,
        descend: bool,
        left: bool,
        right: bool
    }
}

impl MovementRecord {
    pub fn mk(input: &InputManager) -> Self {
        let mut s = Self::default();
        if input.is_key_pressing(Key::Character("w".into())) {
            s.forward = true;
        }
        if input.is_key_pressing(Key::Character("a".into())) {
            s.left = true;
        }
        if input.is_key_pressing(Key::Character("s".into())) {
            s.backward = true;
        }
        if input.is_key_pressing(Key::Character("d".into())) {
            s.right = true;
        }
        if input.is_key_pressing(Key::Named(NamedKey::Space)) {
            s.ascend = true;
        }
        if input.is_key_pressing(Key::Named(NamedKey::Shift)) {
            s.descend = true;
        }
        s
    }
}
