pub mod block;
pub mod block_face_direction;
pub use block::*;
use blockworld_utils::Registry;
use once_cell::sync::Lazy;

pub static BLOCK_REGISTRY: Lazy<Registry<Block>> = Lazy::new(|| {
    let mut r = Registry::new();
    let a0 = Block::new(blockworld_utils::Identifier::new(&format!("{}:air", blockworld_utils::GAME_NAME)));
    r.register(a0);
    let a1 = Block::new(blockworld_utils::Identifier::new(&format!("{}:stone", blockworld_utils::GAME_NAME)));
    r.register(a1);
    let a2 = Block::new(blockworld_utils::Identifier::new(&format!("{}:dirt", blockworld_utils::GAME_NAME)));
    r.register(a2);
    let a3 = Block::new(blockworld_utils::Identifier::new(&format!("{}:grass_block", blockworld_utils::GAME_NAME)));
    r.register(a3);

    r
});
