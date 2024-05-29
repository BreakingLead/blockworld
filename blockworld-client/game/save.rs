//! Save file(Map) manager

use std::collections::{HashMap, HashSet};

use anyhow::*;
use glam::{ivec2, IVec2, IVec3};

use crate::game::chunk::CHUNK_SIZE;

use super::{block::Block, chunk::Chunk};
struct ChunkPool {
    chunks: HashMap<IVec2, Chunk>,
}

impl ChunkPool {
    pub fn new() -> Self {
        ChunkPool {
            chunks: HashMap::new(),
        }
    }

    pub fn load_chunk(x: i32, y: i32) -> Result<()> {
        todo!()
    }

    pub fn generate_chunk(&mut self, x: i32, z: i32) -> Result<()> {
        if self.chunks.contains_key(&ivec2(x, z)) {
            Err(anyhow::Error::msg("Chunk already generated"))
        } else {
            let mut chunk = Chunk::default();
            chunk.x_pos = x;
            chunk.z_pos = x;
            self.chunks.insert(ivec2(x, z), chunk);
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
