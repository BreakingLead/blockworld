//! Block registry — Minecraft 1.12.2-era common blocks.
//!
//! Blocks are identified by `{GAME_NAME}:{name}` and mapped to numeric IDs.
//! The texture atlas resolves block IDs to textures via `query_uv()` fallback
//! (tries `{name}_top` if `{name}` isn't found directly).

pub mod block;
pub mod block_face_direction;
pub use block::*;
use blockworld_utils::{Identifier, Registry};
use once_cell::sync::Lazy;

macro_rules! register {
    ($r:expr, $name:literal) => {
        $r.register(Block::new(Identifier::new(&format!(
            "{}:{}",
            blockworld_utils::GAME_NAME,
            $name
        ))));
    };
}

pub static BLOCK_REGISTRY: Lazy<Registry<Block>> = Lazy::new(|| {
    let mut r = Registry::new();

    // -- air (ID 0, the default) --
    register!(r, "air");

    // -- natural terrain --
    register!(r, "stone");
    register!(r, "dirt");
    register!(r, "grass_block");
    register!(r, "sand");
    register!(r, "gravel");
    register!(r, "clay");
    register!(r, "ice");
    register!(r, "snow");
    register!(r, "bedrock");
    register!(r, "obsidian");

    // -- wood --
    register!(r, "oak_planks");
    register!(r, "spruce_planks");
    register!(r, "birch_planks");
    register!(r, "jungle_planks");
    register!(r, "acacia_planks");
    register!(r, "dark_oak_planks");
    register!(r, "oak_log");
    register!(r, "spruce_log");
    register!(r, "birch_log");
    register!(r, "jungle_log");
    register!(r, "oak_leaves");
    register!(r, "spruce_leaves");
    register!(r, "birch_leaves");

    // -- stone variants --
    register!(r, "cobblestone");
    register!(r, "mossy_cobblestone");
    register!(r, "stone_bricks");
    register!(r, "bricks");

    // -- ores --
    register!(r, "coal_ore");
    register!(r, "iron_ore");
    register!(r, "gold_ore");
    register!(r, "diamond_ore");
    register!(r, "emerald_ore");
    register!(r, "redstone_ore");
    register!(r, "lapis_ore");

    // -- utility --
    register!(r, "crafting_table");
    register!(r, "furnace");
    register!(r, "bookshelf");
    register!(r, "chest");

    // -- nether --
    register!(r, "netherrack");
    register!(r, "soul_sand");
    register!(r, "glowstone");
    register!(r, "nether_bricks");

    // -- end --
    register!(r, "end_stone");

    // -- glass --
    register!(r, "glass");

    r
});
