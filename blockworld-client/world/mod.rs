use bevy_ecs::world::World;
use chunk_array::ChunkArray;
use once_cell::sync::Lazy;

use crate::renderer::entity::Player;

pub mod chunk;
pub mod chunk_array;

pub struct Blockworld {
    chunks: ChunkArray,
    ecs: World,
}

impl Blockworld {
    pub fn new() -> Self {
        Self {
            chunks: ChunkArray::new(8),
            ecs: World::default(),
        }
    }
}

static mut BLOCKWORLD: Lazy<Blockworld> = Lazy::new(|| Blockworld::new());
