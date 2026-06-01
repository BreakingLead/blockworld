use blockworld_server::world::disk_chunk_access::DiskChunkArray;

pub struct BlockworldClient {
    pub chunks: DiskChunkArray,
}

impl BlockworldClient {
    pub fn new() -> Self {
        Self {
            chunks: DiskChunkArray::new(4),
        }
    }
}
