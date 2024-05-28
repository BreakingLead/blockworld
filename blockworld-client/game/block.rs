use std::default;

use glam::Vec3;

use crate::io::atlas_helper::AtlasCoordinate;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct ResourceLocation(&'static str);

pub type BlockID = ResourceLocation;
#[derive(Default, Clone, Copy)]
pub struct Block {
    pub id: BlockID,
}

/// Metadata for query from id
pub struct BlockMeta {
    pub name: ResourceLocation,
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
