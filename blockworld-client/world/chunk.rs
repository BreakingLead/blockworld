use std::{
    borrow::{Borrow, Cow},
    ops::Div,
};

use bevy_ecs::system::Res;
use blockworld_utils::ResourceLocation;
use enumflags2::{BitFlag, BitFlags};
use glam::*;
use wgpu::core::storage;

use crate::{block::*, renderer::block::BlockFaceDirection};

pub const SUBCHUNK_SIZE: usize = 16;
pub const SUBCHUNK_BLOCK_NUM: usize = SUBCHUNK_SIZE * SUBCHUNK_SIZE * SUBCHUNK_SIZE;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_BLOCK_NUM: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

// ! Should be optimized later by using 4 bit instead of u8
type LightLevel = u8;

pub struct SubChunk {
    // block_palette: Vec<NumberID>,
    // in yzx order
    // can be empty
    // blocks: Option<Vec<u16>>,

    // temp, low performance
    blocks: Box<[u32; 4096]>,
}

impl SubChunk {
    pub fn new() -> Self {
        Self {
            // block_palette: Vec::new(),
            blocks: Box::new([0; 4096]),
        }
    }

    /// Reference: [https://minecraft.wiki/w/Chunk_format]
    ///
    /// Format: YZX
    ///
    /// From xyz to Index of the block array.
    ///
    fn index(x: i32, y: i32, z: i32) -> usize {
        // Make sure the index is in the range of 0..15
        assert!(
            x >= 0
                && y >= 0
                && z >= 0
                && x <= SUBCHUNK_SIZE as i32
                && y <= SUBCHUNK_SIZE as i32
                && z <= SUBCHUNK_SIZE as i32
        );

        (y * CHUNK_SIZE as i32 * CHUNK_SIZE as i32 + z * CHUNK_SIZE as i32 + x) as usize
    }

    pub fn set_blockid(&mut self, x: i32, y: i32, z: i32, block_id: &str) {
        let number_id = BLOCK_REGISTRY.name_to_number_id(&block_id.into());
        self.blocks[Self::index(x, y, z)] = number_id;
    }

    pub fn remove_block(&mut self, x: i32, y: i32, z: i32) {
        self.blocks[Self::index(x, y, z)] = 0;
    }

    pub fn get_blockid(&self, x: i32, y: i32, z: i32) -> &'static str {
        if let Some(r) = BLOCK_REGISTRY.number_id_to_name(self.blocks[Self::index(x, y, z)]) {
            r
        } else {
            "minecraft:air"
        }
    }
}

pub struct Chunk {
    /// Blocks
    pub storage_array: [Box<SubChunk>; 16],

    pub pos: IVec2,
    /// Should update this when the chunk is modified
    pub is_modified: bool,
    pub is_chunk_loaded: bool,
    pub height_map: [i32; CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    pub fn new(pos_x: i32, pos_z: i32) -> Self {
        // initialize the subchunk array
        let storage_array = core::array::from_fn(|_| Box::new(SubChunk::new()));
        Self {
            storage_array,
            pos: ivec2(pos_x, pos_z),
            is_modified: false,
            is_chunk_loaded: false,
            height_map: [0; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    pub fn is_air(&self, x: i32, y: i32, z: i32) -> bool {
        self.get_block_id(x, y, z) == "minecraft:air"
    }

    /// Get the block at (x,y,z) with respect to the chunk-relative coord.
    pub fn get_block_id(&self, x: i32, y: i32, z: i32) -> &'static str {
        let subchunk_index = y.div(SUBCHUNK_SIZE as i32) as usize;

        self.storage_array[subchunk_index].get_blockid(x, y % SUBCHUNK_SIZE as i32, z)
    }

    pub fn set_block_id(&mut self, x: i32, y: i32, z: i32, block_id: &'static str) {
        let subchunk_index = y.div(SUBCHUNK_SIZE as i32) as usize;

        self.storage_array[subchunk_index].set_blockid(x, y % SUBCHUNK_SIZE as i32, z, &block_id);
        self.is_modified = true;
    }

    pub fn remove_block(&mut self, x: i32, y: i32, z: i32) {
        let subchunk_index = y.div(SUBCHUNK_SIZE as i32) as usize;

        self.storage_array[subchunk_index].remove_block(x, y % SUBCHUNK_SIZE as i32, z);
        self.is_modified = true;
    }

    /// for face cull
    /// this should be in worldwide because one chunk can't see another chunk
    /// but i put here for now for a while
    pub fn exist_neighbor(&self, x: i32, y: i32, z: i32) -> BitFlags<BlockFaceDirection> {
        let mut m = BitFlags::empty();
        if x == 0
            || x == CHUNK_SIZE as i32 - 1
            || y == 0
            || y == CHUNK_HEIGHT as i32 - 1
            || z == 0
            || z == CHUNK_SIZE as i32 - 1
        {
            BlockFaceDirection::empty()
        } else {
            if !self.is_air(x - 1, y, z) {
                m |= BlockFaceDirection::XN;
            }
            if !self.is_air(x + 1, y, z) {
                m |= BlockFaceDirection::XP;
            }
            if !self.is_air(x, y - 1, z) {
                m |= BlockFaceDirection::YN;
            }
            if !self.is_air(x, y + 1, z) {
                m |= BlockFaceDirection::YP;
            }
            if !self.is_air(x, y, z - 1) {
                m |= BlockFaceDirection::ZN;
            }
            if !self.is_air(x, y, z + 1) {
                m |= BlockFaceDirection::ZP;
            }
            m
        }
    }
}
