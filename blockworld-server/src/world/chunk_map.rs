//! Chunk storage — a `HashMap<IVec3, SubChunk>` with view management
//! and render invalidation. Implements `WorldAccess`.

use std::collections::HashMap;

use blockworld_utils::Identifier;
use glam::IVec3;

use crate::packet::Packet;

use super::{chunk::SubChunk, chunk::world_blockpos_to_chunkpos, chunk_access::WorldAccess};

/// In-memory chunk cache.
///
/// Tracks which subchunks are loaded, which need mesh regeneration,
/// and handles block-level queries forwarded through `WorldAccess`.
///
/// Future: will be backed by a disk/region-file layer for persistence.
#[derive(Default)]
pub struct ChunkMap {
    pub chunks: HashMap<IVec3, SubChunk>,
    /// Chunk positions whose render mesh is stale.
    pub need_rerender: Vec<IVec3>,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a block position is air.
    ///
    /// Unloaded chunks are treated as air (nothing to collide with).
    pub fn is_air_block(&self, pos: IVec3) -> bool {
        let (chunk_pos, _) = world_blockpos_to_chunkpos(pos);
        if !self.chunks.contains_key(&chunk_pos) {
            return true;
        }
        self.get_block(pos)
            == Identifier::new(&format!("{}:air", blockworld_utils::GAME_NAME))
    }

    /// Get the block identifier at a world position.
    pub fn get_block(&self, pos: IVec3) -> Identifier {
        let (chunk_pos, block_pos) = world_blockpos_to_chunkpos(pos);
        self.chunks[&chunk_pos].get_blockid(block_pos).into()
    }

    /// Set a block at a world position. Flags the chunk for remesh.
    pub fn set_block(&mut self, pos: IVec3, id: &Identifier) {
        let (chunk_pos, block_pos) = world_blockpos_to_chunkpos(pos);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_blockid(block_pos, &id.to_string());
            self.need_rerender.push(chunk_pos);
        }
    }
}

// -- WorldAccess impl -------------------------------------------------------

impl WorldAccess for ChunkMap {
    fn is_chunk_loaded(&self, pos: IVec3) -> bool {
        self.chunks.contains_key(&pos)
    }

    fn get_chunk(&self, pos: IVec3) -> &SubChunk {
        self.chunks
            .get(&pos)
            .expect("Chunk not loaded - call is_chunk_loaded() first")
    }

    fn load_chunk(&mut self, pos: IVec3) {
        if !self.chunks.contains_key(&pos) {
            self.need_rerender.push(pos);
            self.chunks.insert(pos, SubChunk::new(pos));
        }
    }

    fn unload_chunk(&mut self, pos: IVec3) {
        self.need_rerender
            .swap_remove(self.need_rerender.iter().position(|x| *x == pos).unwrap());
        self.chunks.remove(&pos);
    }

    fn iter_loaded_chunks(&self) -> impl Iterator<Item = &SubChunk> {
        self.chunks.values()
    }

    fn update(&mut self, packet: Packet) {
        if let Packet::BlockUpdate(pos, id) = packet {
            let (chunk_pos, _) = world_blockpos_to_chunkpos(pos);
            if self.is_chunk_loaded(chunk_pos) {
                self.need_rerender.push(chunk_pos);
                let chunk = self.chunks.get_mut(&chunk_pos).unwrap();
                chunk.set_blockid(pos, &id);
            }
        }
    }

    fn is_air(&self, pos: IVec3) -> bool {
        self.is_air_block(pos)
    }

    fn get_block(&self, pos: IVec3) -> Identifier {
        self.get_block(pos)
    }

    fn set_block(&mut self, pos: IVec3, id: &Identifier) {
        self.set_block(pos, id)
    }

    fn need_rerender(&self, pos: IVec3) -> bool {
        self.need_rerender.contains(&pos)
    }

    fn clear_need_rerender(&mut self, pos: IVec3) {
        self.need_rerender.retain(|&p| p != pos);
    }
}
