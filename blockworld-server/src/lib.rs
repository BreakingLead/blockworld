use world::disk_chunk_access::DiskChunkArray;

pub mod block;
pub mod packet;
pub mod world;

pub struct Blockworld {
    pub chunks: DiskChunkArray,
}

impl Blockworld {
    pub fn new() -> Self {
        Self {
            chunks: DiskChunkArray::new(8),
        }
    }
}
