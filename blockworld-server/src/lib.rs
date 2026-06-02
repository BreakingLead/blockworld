use world::chunk_map::ChunkMap;

pub mod block;
pub mod packet;
pub mod world;

pub struct Blockworld {
    pub chunks: ChunkMap,
}

impl Blockworld {
    pub fn new() -> Self {
        Self {
            chunks: ChunkMap::new(),
        }
    }
}
