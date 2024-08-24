//! ```
//! package net.minecraft.client.renderer.texture
//! class SpriteContents
//! version 1.21
//! ```

use blockworld_utils::ResourceLocation;

/// This is a wrapper around an image::RgbaImage that contains the contents of a sprite. It also will handle its mipmaps.
///
pub struct SpriteContents {
    name: ResourceLocation,
    original_image: image::RgbaImage,
    /// Mipmaps of the image, if they were generated.
    by_mip_level: Option<Vec<image::RgbaImage>>,
}

impl SpriteContents {
    pub fn new(name: ResourceLocation, original_image: image::RgbaImage) -> Self {
        Self {
            name,
            original_image,
            by_mip_level: None,
        }
    }

    pub fn name(&self) -> &ResourceLocation {
        &self.name
    }

    pub fn width(&self) -> u32 {
        self.original_image.width()
    }

    pub fn height(&self) -> u32 {
        self.original_image.height()
    }
}
