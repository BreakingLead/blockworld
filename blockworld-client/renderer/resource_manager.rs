use once_cell::sync::Lazy;

use super::atlas_image::Atlas;
use crate::resource::RESOURCE_MANAGER;

pub static BLOCK_ATLAS: Lazy<Atlas> = Lazy::new(|| {
    let rm = RESOURCE_MANAGER.lock().unwrap();
    Atlas::from_resource_manager(&rm, "minecraft", "textures/block")
});
