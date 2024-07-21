use crate::io::atlas_helper::UV;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceLocation {
    namespace: String,
    value: String,
}

impl ResourceLocation {
    pub fn new(id: &str) -> Self {
        if let Some((a, b)) = id.split_once(':') {
            Self {
                namespace: a.to_string(),
                value: b.to_string(),
            }
        } else {
            Self {
                namespace: "blockworld".to_string(),
                value: id.to_string(),
            }
        }
    }
}

pub type BlockID = u32;
#[derive(Default, Clone, Copy, Debug)]
pub struct Block {
    pub id: BlockID,
}

/// Metadata for query from id
#[derive(Debug)]
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
    pub atlas_coord: [UV; 6],
}

#[derive(Debug, Default, Clone, Copy)]
pub enum BlockType {
    #[default]
    Solid,
    Glass,
}
