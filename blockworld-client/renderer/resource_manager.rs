use blockworld_utils::ResourceLocation;
use once_cell::sync::Lazy;

use super::atlas_image::Atlas;

/// temporarily use this global variable to store the block atlas
///
/// TODO: use a resource manager to load and store atlases
pub static BLOCK_ATLAS: Lazy<Atlas> = Lazy::new(|| Atlas::new("assets/minecraft/textures/block/"));
