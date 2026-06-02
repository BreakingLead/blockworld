pub mod block;
pub mod block_face_direction;
pub use block::*;
use blockworld_utils::{Identifier, Registry};
use once_cell::sync::Lazy;

pub static BLOCK_REGISTRY: Lazy<Registry<Block>> = Lazy::new(|| {
    let mut r = Registry::new();
    let a0 = Block::new(Identifier::new(&format!("{}:air", blockworld_utils::GAME_NAME)));
    r.register(a0);
    let a1 = Block::new(Identifier::new(&format!("{}:stone", blockworld_utils::GAME_NAME)));
    r.register(a1);
    let a2 = Block::new(Identifier::new(&format!("{}:dirt", blockworld_utils::GAME_NAME)));
    r.register(a2);
    let a3 = Block::new(Identifier::new(&format!("{}:grass_block_top", blockworld_utils::GAME_NAME)));
    r.register(a3);

    r
});
