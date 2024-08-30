use chunk_array::ChunkArray;

use crate::renderer::entity::Player;

pub mod block_access;
pub mod chunk;
pub mod chunk_array;

pub struct World {
    chunks: ChunkArray,
    player: Player,
}
