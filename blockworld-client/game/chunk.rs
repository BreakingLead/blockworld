use glam::*;

use crate::render::render_block::*;

use super::block::Block;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_BLOCK_NUM: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

/// from `net/minecraft/util/math/ChunkPos.java`
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn as_long(x: i32, z: i32) -> i64 {
        //? From minecraft source code
        return (x as i64 & 0xFFFFFFFFi64) | (z as i64 & 0xFFFFFFFFi64) << 32;
    }

    pub fn hash_code(&self) -> i32 {
        let i: i32 = 1664525 * self.x + 1013904223;
        let j: i32 = 1664525 * (self.z ^ -559038737) + 1013904223;
        i ^ j
    }
}

pub struct Chunk {
    pub blocks: Box<[Block; CHUNK_HEIGHT * CHUNK_SIZE * CHUNK_SIZE]>,
    pub pos: ChunkPos,
}

impl Chunk {
    /// Reference: [https://minecraft.wiki/w/Chunk_format]
    ///
    /// Format: YZX
    ///
    /// From xyz to Index of the block array.
    ///
    /// Don't pass negative numbers into this function!
    pub fn index(x: i32, y: i32, z: i32) -> usize {
        (y * 16 * 16 + z * 16 + x) as usize
    }

    /// Get the block at (x,y,z) with respect to the chunk-relative coord
    pub fn block(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        if 0 <= x
            && x <= (CHUNK_SIZE - 1) as i32
            && 0 <= y
            && y <= (CHUNK_HEIGHT - 1) as i32
            && 0 <= z
            && z <= (CHUNK_SIZE - 1) as i32
        {
            Some(self.blocks[Chunk::index(x, y, z)])
        } else {
            None
        }
    }

    /// Detect neighbors for face cull
    /// Output format in [AxisDirectionBinary]
    pub fn detect_block_neighbors(&self, x: i32, y: i32, z: i32) -> AxisDirectionBinary {
        let mut res = 0;
        // if x == 0 {
        //     return XN_B;
        // }
        // if x == CHUNK_SIZE - 1 {
        //     return XP_B;
        // }
        // if y == 0 {
        //     return YN_B;
        // }
        // if y == CHUNK_HEIGHT - 1 {
        //     return YP_B;
        // }
        // if z == 0 {
        //     return ZN_B;
        // }
        // if z == CHUNK_SIZE - 1 {
        //     return ZP_B;
        // }

        // Zero = Air
        if self.block(x + 1, y, z).unwrap_or_default().id != 0 {
            res += XP_B;
        }
        if self.block(x - 1, y, z).unwrap_or_default().id != 0 {
            res += XN_B;
        }
        if self.block(x, y + 1, z).unwrap_or_default().id != 0 {
            res += YP_B;
        }
        if self.block(x, y - 1, z).unwrap_or_default().id != 0 {
            res += YN_B;
        }
        if self.block(x, y, z + 1).unwrap_or_default().id != 0 {
            res += ZP_B;
        }
        if self.block(x, y, z - 1).unwrap_or_default().id != 0 {
            res += ZN_B;
        }
        return res;
    }
}

impl Default for Chunk {
    // THIS IS NOT IDEAL
    // JUST FOR TEST
    // REMEMBER TO DELETE THOSE CODE
    fn default() -> Self {
        let mut blocks = Box::new([Block::default(); CHUNK_BLOCK_NUM]);
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..3 {
                for z in 0..CHUNK_SIZE as i32 {
                    blocks[Chunk::index(x, y, z)] = match y {
                        _ => Block { id: 1 },
                    }
                }
            }
        }
        for x in 0..CHUNK_SIZE as i32 {
            for y in 5..40 {
                for z in 0..CHUNK_SIZE as i32 {
                    if (vec3(x as f32, y as f32, z as f32) - vec3(7.0, 15.0, 7.0)).length() <= 7.0 {
                        blocks[Chunk::index(x, y, z)] = match y {
                            _ => Block { id: 2 },
                        }
                    }
                }
            }
        }

        Self {
            blocks,
            pos: ChunkPos::new(0, 0),
        }
    }
}
