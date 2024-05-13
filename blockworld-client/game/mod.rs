use std::collections::HashMap;

use glam::{IVec2, Vec2};

pub mod block;
pub mod chunk;
pub mod player_state;

// Single Instance Mode
// ! The value of the hashmap is temporary.
pub struct RegisterTable {
    pub table_block: HashMap<String, (IVec2)>,
}
