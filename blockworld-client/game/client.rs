use blockworld_server::world::chunk_generator::ChunkGenerator;
use blockworld_server::world::chunk_map::ChunkMap;

pub struct BlockworldClient {
    pub chunks: ChunkMap,
}

impl BlockworldClient {
    pub fn new() -> Self {
        Self {
            chunks: ChunkMap::new(),
        }
    }

    /// Generate terrain in a 3×3 area around the origin.
    pub fn generate_initial_terrain(&mut self) {
        for x in -1..=1 {
            for z in -1..=1 {
                ChunkGenerator::generate(&mut self.chunks, glam::ivec3(x, 0, z));
            }
        }
        log::info!("Generated {} chunks", self.chunks.chunks.len());
    }
}
