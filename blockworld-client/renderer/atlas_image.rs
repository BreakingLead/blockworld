//! ```
//! package net.minecraft.client.renderer.texture
//! class SpriteContents
//! version 1.21
//! ```

use std::{collections::HashMap, fmt::Display, path::Path};

use blockworld_utils::ResourceLocation;
use glam::{ivec2, uvec2, vec2, IVec2, UVec2, Vec2};
use image::{GenericImage, GenericImageView, ImageBuffer};
use wgpu::hal::auxil::db;

/// This is a wrapper around an image::RgbaImage that contains the contents of a sprite. It also will handle its mipmaps.
pub struct Atlas {
    /// - "minecraft:atlas/block"
    /// - "minecraft:atlas/item"
    /// - "ic2:atlas/item"
    /// - etc.
    // self_name: ResourceLocation,
    atlas: image::RgbaImage,
    /// Mipmaps of the image, if they were generated.
    by_mip_level: Option<Vec<image::RgbaImage>>,

    tile_size: u32,
    name_to_xy_map: HashMap<ResourceLocation, UVec2>,
}

impl Atlas {
    pub fn new<Q: AsRef<Path>>(assets_path: Q) -> Self {
        let width_pixels = 512;
        let height_pixels = 512;
        let tile_size = 16;
        let count_per_row = width_pixels / tile_size;
        let mut atlas = ImageBuffer::new(width_pixels, height_pixels);

        // there is an optional .mcmeta file of a texture
        // e.g. textures/blocks/grass_block.png with textures/blocks/grass_block.png.mcmeta
        // read every png and ignore the pngs with optional .mcmeta, since we haven't finished
        // implementing the mcmeta parsing yet.

        // in this function we just use the picture's name as the resource location,
        // and it's not ideal since we haven't implemented reading resource packs.

        log::warn!(
            "Creating new texture atlas, reading from {:?}",
            assets_path.as_ref()
        );

        let mut name_to_xy_map = HashMap::new();
        let mut counter = 0;
        for entry in assets_path.as_ref().read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file()
                && path.extension().unwrap() == "png"
                && !path.join(".mcmeta").exists()
            {
                let x = counter as u32 % count_per_row;
                let y = counter as u32 / count_per_row;
                let img = image::open(&path).unwrap();

                if img.dimensions().0 > tile_size || img.dimensions().1 > tile_size {
                    // TODO: read meta, then reimpelement this
                    log::warn!(
                        "Image {} is too big for the tile size, ignoring",
                        path.display()
                    );
                    continue;
                }

                atlas.copy_from(&img, x * tile_size, y * tile_size).unwrap();

                let item_name = path.file_stem().unwrap();
                let r = ResourceLocation::new(item_name.to_str().unwrap());

                name_to_xy_map.insert(r, uvec2(x, y));

                counter += 1;
            }
        }

        Self {
            atlas,
            by_mip_level: None,
            tile_size,
            name_to_xy_map,
        }
    }

    pub fn get_image(&self) -> &image::RgbaImage {
        &self.atlas
    }

    pub fn save<Q>(&self, root: Q)
    where
        Q: AsRef<Path>,
    {
        self.atlas.save(root).unwrap();
    }

    fn width(&self) -> u32 {
        self.atlas.width()
    }

    fn height(&self) -> u32 {
        self.atlas.height()
    }

    fn from_xy_to_uvs(&self, xy: UVec2) -> (Vec2, Vec2) {
        let x = xy.x;
        let y = xy.y;
        assert!(
            x < self.width() / self.tile_size && y < self.height() / self.tile_size,
            "xy out of bounds"
        );
        let u1 = x * self.tile_size;
        let v1 = y * self.tile_size;
        let u2 = u1 + self.tile_size;
        let v2 = v1 + self.tile_size;
        (
            vec2(
                u1 as f32 / (self.width() as f32),
                v1 as f32 / (self.height() as f32),
            ),
            vec2(
                u2 as f32 / (self.width() as f32),
                v2 as f32 / (self.height() as f32),
            ),
        )
    }

    pub fn query_uv(&self, name: &ResourceLocation) -> Option<(Vec2, Vec2)> {
        let xy = self.name_to_xy_map.get(name).cloned()?;
        Some(self.from_xy_to_uvs(xy))
    }
}

impl Display for Atlas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Atlas({:?})", self.atlas.dimensions())
    }
}

#[test]
fn atlas_generation() {
    let atlas = Atlas::new(Path::new("assets/minecraft/textures/block"));
    dbg!(&atlas.name_to_xy_map);
    atlas.save("run/");
}
