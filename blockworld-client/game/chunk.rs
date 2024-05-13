use glam::{ivec2, vec3, IVec2};

use super::block::{Block, BlockType};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub struct Chunk {
    pub blocks: Box<[Block; CHUNK_HEIGHT * CHUNK_SIZE * CHUNK_SIZE]>,
    pub coord: IVec2,
}

impl Chunk {
    pub fn index_from_xyz(x: usize, y: usize, z: usize) -> usize {
        z + x * 16 + y * 256
    }

    // THIS IS BUGGY
    // REMEMBER TO FIX IT
    pub fn new() -> Self {
        let mut blocks = Box::new([Block::default(); 65536]);
        for x in (0..CHUNK_SIZE) {
            for y in (0..1) {
                for z in (0..CHUNK_SIZE) {
                    blocks[Self::index_from_xyz(x, y, z)] = Block::new(BlockType::Stone);
                }
            }
        }
        Self {
            blocks,
            coord: ivec2(0, 0),
        }
    }
}
