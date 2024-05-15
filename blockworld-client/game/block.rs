use std::default;

use glam::Vec3;

use crate::render::texture::AtlasCoordinate;

pub type BlockID = u32;
#[derive(Default, Clone, Copy)]
pub struct Block {
    pub id: BlockID,
}

/// Metadata for query from id
#[derive(Debug)]
pub struct BlockMeta {
    pub name: String,
    pub ty: BlockType,
    /// Attention:
    /// - 0: Up
    /// - 1: Down
    /// - 2: Left
    /// - 3: Right
    /// - 4: Front
    /// - 5: Back
    pub atlas_coord: [AtlasCoordinate; 6],
}

#[derive(Debug, Default, Clone, Copy)]
pub enum BlockType {
    #[default]
    Solid,
    Glass,
}
