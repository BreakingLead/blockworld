use std::default;

use glam::Vec3;

#[derive(Default, Clone, Copy)]
pub struct Block {
    // This means "name" but i leave it as domain for future usage ("blockworld:something").
    pub domain: BlockType,
}

#[derive(Default, Clone, Copy)]
pub enum BlockType {
    #[default]
    Air,
    Stone,
}

impl Block {
    pub fn new(domain: BlockType) -> Self {
        Self { domain }
    }
}
