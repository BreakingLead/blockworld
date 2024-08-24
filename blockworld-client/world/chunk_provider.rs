//! net/minecraft/client/multiplayer/ClientChunkProvider.java
#[feature(int_roundings)]
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Result};
use blockworld_utils::*;

use crate::game::world::ClientWorld;

pub struct ClientChunkProvider {
    array: ChunkArray,
    world: AM<ClientWorld>,
}

impl ClientChunkProvider {
    pub fn new(world: AM<ClientWorld>, view_distance: i32) -> Self {
        let array = ChunkArray::new(view_distance);
        Self { array, world }
    }

    fn unload_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<()> {
        if self.array.in_view(chunk_x, chunk_z) {
            let index = self.array.get_index(chunk_x, chunk_z);
            let chunk = self.array.get(index);
            // ! fix this unwrap, validate chunk before unload
            self.array.unload(index, chunk.unwrap())?;
        }
        Ok(())
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<AM<Chunk>> {
        if self.array.in_view(chunk_x, chunk_z) {
            let chunk = self.array.get(self.array.get_index(chunk_x, chunk_z));
            // if Self::is_valid(&chunk, chunk_x, chunk_z) {
            if true {
                chunk
            } else {
                None
            }
        } else {
            None
        }
    }

    // ! NOT COMPLETE
    pub fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Option<AM<Chunk>> {
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
            *chunk = Some(Arc::new(Mutex::new((Chunk::new(chunk_x, chunk_z)))));

            return Some(chunk.as_ref().unwrap().clone());
        }
    }
}

/// The place which holds references of all loaded chunks
struct ChunkArray {
    /// Stored references
    chunks: Vec<Option<AM<Chunk>>>,
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

    pub fn get(&self, chunk_index: usize) -> Option<AM<Chunk>> {
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
    /// # Panics
    /// If the chunk is not in the view distance or the index is out of bound.
    ///
    pub fn unload(&mut self, chunk_index: usize, chunk_in: AM<Chunk>) {
        if self.get(chunk_index).is_none() {
            panic!("target chunk does not exist");
        } else if !Arc::ptr_eq(&self.get(chunk_index).unwrap(), &chunk_in) {
            panic!("target chunk does not match the index")
        } else {
            self.chunks[chunk_index] = None;
            self.loaded -= 1;
        }
    }
}
