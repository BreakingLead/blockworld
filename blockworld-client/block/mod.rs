mod block;
pub use block::*;
use blockworld_utils::Registry;
use once_cell::sync::Lazy;

pub static BLOCK_REGISTRY: Lazy<Registry<&'static dyn Block>> = Lazy::new(|| {
    let mut r = Registry::new();

    let mut number_id = 0;
    r.register("air".into(), &Air as &dyn Block);

    number_id += 1;
    r.register("stone".into(), &Stone);

    number_id += 1;
    r.register("grass".into(), &Grass);
    r
});
