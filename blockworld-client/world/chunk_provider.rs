//! net/minecraft/client/multiplayer/ClientChunkProvider.java
use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};

trait ChunkProvider {
    /**
     * Checks to see if a chunk exists at x, y
     */
    fn chunk_exists(&self, chunk_x: i32, chunk_z: i32) -> bool;

    /**
     * Will return back a chunk, if it doesn't exist and its not a MP client it will generates all the blocks for the
     * specified chunk from the map seed and chunk seed
     */
    fn provide_chunk(&self, chunk_x: i32, chunk_z: i32) -> Rc<RefCell<Chunk>>;

    /**
     * loads or generates the chunk at the chunk location specified
     */
    fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Rc<RefCell<Chunk>> {
        self.provide_chunk(chunk_x, chunk_z)
    }
}

use super::{chunk::Chunk, world::ClientWorld};
pub struct ClientChunkProvider {
    array: ChunkArray,
    world: Rc<ClientWorld>,
}

impl ClientChunkProvider {
    pub fn new(world: Rc<ClientWorld>, view_distance: i32) -> Self {
        let array = ChunkArray::new(view_distance);
        Self { array, world }
    }

    fn is_valid(chunk_in: &Option<Rc<Chunk>>, x: i32, z: i32) -> bool {
        if let Some(chunk) = chunk_in {
            let pos = chunk.pos;
            pos.x == x && pos.z == z
        } else {
            false
        }
    }

    fn unload_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<()> {
        if self.array.in_view(chunk_x, chunk_z) {
            let index = self.array.get_index(chunk_x, chunk_z);
            let chunk = self.array.get(index);
            if Self::is_valid(&chunk, chunk_x, chunk_z) {
                // this unwrap is safe because it's checked to be vaild
                self.array.unload(index, chunk.unwrap())?;
            }
        }
        Ok(())
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Rc<Chunk>> {
        if self.array.in_view(chunk_x, chunk_z) {
            let chunk = self.array.get(self.array.get_index(chunk_x, chunk_z));
            if Self::is_valid(&chunk, chunk_x, chunk_z) {
                chunk
            } else {
                None
            }
        } else {
            None
        }
    }

    // ! NOT COMPLETE
    pub fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Option<Rc<Chunk>> {
        if !self.array.in_view(chunk_x, chunk_z) {
            log::error!(
                "Ignoring chunk since we don't have complete data: {}, {}",
                chunk_x,
                chunk_z
            );
            return None;
        } else {
            let index = self.array.get_index(chunk_x, chunk_z);
            if index >= self.array.chunks.len() {
                log::error!("chunk index out of the bound");
                return None;
            }
            let chunk = &mut self.array.chunks[index];
            *chunk = Some(Rc::new(Chunk::new(chunk_x, chunk_z)));

            return Some(chunk.as_ref().unwrap().clone());
        }
    }
}

/// The place which holds references of all loaded chunks
struct ChunkArray {
    /// Stored references
    chunks: Vec<Option<Rc<Chunk>>>,
    /// Set this with the view distance
    view_distance: i32,
    side_length: i32,
    /// X Coord of the center chunk which is where we are in
    center_x: i32,
    /// Z Coord of the center chunk which is where we are in
    center_z: i32,
    /// The count of loaded chunks
    loaded: i32,
}

impl ChunkArray {
    /// Create a default chunk array with view distance.
    ///
    /// Look up for settings to get the view distance.
    pub fn new(view_distance: i32) -> Self {
        let side_length = view_distance * 2 + 1;
        Self {
            view_distance,
            side_length,
            chunks: vec![None; (side_length * side_length) as usize],
            center_x: 0,
            center_z: 0,
            loaded: 0,
        }
    }

    /// From chunk coord to index of the array.
    pub fn get_index(&self, chunk_x: i32, chunk_z: i32) -> usize {
        let floor_mod = |a: i32, b: i32| a - a.div_floor(b) * b;
        (floor_mod(chunk_z, self.side_length) * self.side_length
            + floor_mod(chunk_x, self.side_length)) as usize
    }

    pub fn get(&self, chunk_index: usize) -> Option<Rc<Chunk>> {
        if chunk_index >= self.chunks.len() {
            None
        } else {
            self.chunks[chunk_index].clone()
        }
    }

    /// Check if the chunk [x, z] is in the view distance.
    pub fn in_view(&self, chunk_x: i32, chunk_z: i32) -> bool {
        (chunk_x - self.center_x).abs() <= self.view_distance
            && (chunk_z - self.center_z).abs() <= self.view_distance
    }

    // ! Unfinished
    /// chunk's index must match itself
    pub fn unload(&mut self, chunk_index: usize, chunk_in: Rc<Chunk>) -> Result<()> {
        if self.get(chunk_index).is_none() {
            Err(anyhow!("target chunk does not exist"))
        } else if !Rc::ptr_eq(&self.get(chunk_index).unwrap(), &chunk_in) {
            Err(anyhow!("target chunk does not match the index"))
        } else {
            self.chunks[chunk_index] = None;
            self.loaded -= 1;
            Ok(())
        }
    }
}
