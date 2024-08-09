use std::{ops::Div, rc::Rc};

use glam::*;

use crate::block::*;

use super::{chunk_provider::ClientChunkProvider, world::World};

pub const SUBCHUNK_SIZE: usize = 16;
pub const SUBCHUNK_BLOCK_NUM: usize = SUBCHUNK_SIZE * SUBCHUNK_SIZE * SUBCHUNK_SIZE;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_BLOCK_NUM: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

// ! Should be optimized later by using 4 bit instead of u8
type LightLevel = u8;

// ExtendedBlockStorage.java
pub struct SubChunk {
    /// A total count of the number of non-air blocks in this block storage's Chunk.
    block_ref_count: u32,
    block_array: [BlockID; CHUNK_BLOCK_NUM],
    light_array: [LightLevel; CHUNK_BLOCK_NUM],
}

impl SubChunk {
    pub fn new() -> Self {
        Self {
            block_ref_count: 0,
            block_array: [0; CHUNK_BLOCK_NUM],
            light_array: [0; CHUNK_BLOCK_NUM],
        }
    }

    pub fn get_blockid(&self, x: i32, y: i32, z: i32) -> BlockID {
        self.block_array[Chunk::index(x, y, z)]
    }
}

pub struct Chunk {
    pub world: Rc<World>,

    /// Blocks
    pub storage_array: [Box<SubChunk>; 16],

    pub pos_x: i32,
    pub pos_z: i32,
    /// Should update this when the chunk is modified
    pub is_modified: bool,
    pub is_chunk_loaded: bool,
    pub height_map: [i32; CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    pub fn new(world: &Rc<World>, pos_x: i32, pos_z: i32) -> Self {
        Self {
            world: Rc::clone(world),
            storage_array: todo!(),
            pos_x,
            pos_z,
            is_modified: false,
            is_chunk_loaded: false,
            height_map: [0; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    /// Reference: [https://minecraft.wiki/w/Chunk_format]
    ///
    /// Format: YZX
    ///
    /// From xyz to Index of the block array.
    ///
    /// Don't pass negative numbers into this function!
    pub fn index(x: i32, y: i32, z: i32) -> usize {
        (y << 8 + z << 4 + x) as usize
    }

    /// Get the block at (x,y,z) with respect to the chunk-relative coord.
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> BlockID {
        let subchunk_index = y.div(SUBCHUNK_SIZE as i32) as usize;
        let block =
            self.storage_array[subchunk_index].get_blockid(x, (y % SUBCHUNK_SIZE as i32) as i32, z);
        block
    }

    /// Detect neighbors for face cull.
    /// The coordinate is chunk-relative.
    pub fn detect_block_neighbors(&self, x: i32, y: i32, z: i32) -> u32 {}
}
