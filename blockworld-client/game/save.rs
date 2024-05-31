//! Save file(Map) manager

use std::collections::HashMap;

use anyhow::*;
use glam::{ivec2, IVec2, IVec3};

use crate::game::chunk::CHUNK_SIZE;

use super::chunk::{Chunk, ChunkPos};
struct ChunkPool {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl ChunkPool {
    pub fn new() -> Self {
        ChunkPool {
            chunks: HashMap::new(),
        }
    }

    pub fn load_chunk(pos: ChunkPos) -> Result<()> {
        let _ = pos;
        todo!();
    }

    pub fn generate_chunk(&mut self, pos: ChunkPos) -> Result<()> {
        if self.chunks.contains_key(&pos) {
            Err(anyhow::Error::msg("Chunk already generated"))
        } else {
            let mut chunk = Chunk::default();
            chunk.pos = pos;
            self.chunks.insert(pos, chunk);
            Ok(())
        }
    }
}

///
/// # Example
/// ```
/// let a = ivec3(3,3,3);
/// assert_eq!(chunk_coord_from_block_coord(a), ivec2(0,0))
/// ```
pub fn chunk_coord_from_block(pos: IVec3) -> IVec2 {
    ivec2(
        (pos.x as f32 / CHUNK_SIZE as f32).floor() as i32,
        (pos.z as f32 / CHUNK_SIZE as f32).floor() as i32,
    )
}
