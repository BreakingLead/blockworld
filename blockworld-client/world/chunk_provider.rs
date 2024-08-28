//! net/minecraft/client/multiplayer/ClientChunkProvider.java
use std::{
    collections::{hash_map::Iter, HashMap},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use blockworld_utils::*;
use glam::IVec2;

use super::chunk::Chunk;

/// The place which holds all loaded chunks
pub struct ChunkArray {
    pub chunks: HashMap<IVec2, Chunk>,
    /// Set this with the view distance
    view_distance: u32,
    /// Coord of the center chunk which is where we are in
    center: IVec2,
    /// The count of loaded chunks
    loaded: u32,
}

impl ChunkArray {
    /// Create a default chunk array with view distance.
    ///
    /// Look up for settings to get the view distance.
    pub fn new(view_distance: u32) -> Self {
        let side_length = (view_distance * 2 + 1) as usize;
        let mut chunks = HashMap::with_capacity(side_length * side_length);
        // TODO: REMOVE THESE TRASH
        for x in -4..=4 {
            for z in -4..=4 {
                let mut c = Chunk::new(x, z);
                for x_c in 0..16 {
                    for z_c in 0..16 {
                        c.set_block_id(x_c, 1, z_c, "blockworld:stone".into());
                    }
                }
                c.is_chunk_loaded = true;
                chunks.insert(IVec2::new(x, z), c);
            }
        }

        Self {
            view_distance,
            chunks,
            center: IVec2::ZERO,
            loaded: 0,
        }
    }

    pub fn add(&mut self, mut chunk: Chunk) {
        chunk.is_chunk_loaded = true;
        self.chunks.insert(chunk.pos, chunk);
        self.loaded += 1;
    }

    pub fn unload(&mut self, chunk_pos: IVec2) {
        if let Some(mut chunk) = self.chunks.remove(&chunk_pos) {
            chunk.is_chunk_loaded = false;
            // TODO: serialize chunk to disk
            self.loaded -= 1;
        } else {
            log::error!("Tried to unload non-existent chunk: {}", chunk_pos);
        }
    }

    fn get(&self, chunk_pos: IVec2) -> Option<&Chunk> {
        self.chunks.get(&chunk_pos)
    }

    fn get_mut(&mut self, chunk_pos: IVec2) -> Option<&mut Chunk> {
        self.chunks.get_mut(&chunk_pos)
    }

    /// Check if the chunk [x, z] is in the view distance.
    pub fn in_view(&self, chunk_x: i32, chunk_z: i32) -> bool {
        (chunk_x - self.center.x).abs() <= self.view_distance as i32
            && (chunk_z - self.center.y).abs() <= self.view_distance as i32
    }
}
