mod block;
pub use block::*;
use blockworld_utils::Registry;
use once_cell::sync::Lazy;

pub static BLOCK_REGISTRY: Lazy<Registry<Block>> = Lazy::new(|| {
    let mut r = Registry::new();
    let a0 = Block::new("minecraft:air".into());
    r.register(a0);
    let a1 = Block::new("minecraft:stone".into());
    r.register(a1);

    r
});
