//! ```
//! package net.minecraft.client.renderer.texture
//! class SpriteContents
//! version 1.21
//! ```

use std::{collections::HashMap, fmt::Display, path::Path};

use blockworld_utils::Identifier;
use glam::{uvec2, vec2, UVec2, Vec2};
use image::{GenericImage, GenericImageView, ImageBuffer};

use crate::resource::ResourceManager;

pub struct Atlas {
    atlas: image::RgbaImage,
    /// Mipmaps of the image, if they were generated.
    by_mip_level: Option<Vec<image::RgbaImage>>,

    tile_size: u32,
    name_to_xy_map: HashMap<Identifier, UVec2>,
}

impl Atlas {
    pub fn from_resource_manager(
        rm: &ResourceManager,
        namespace: &str,
        prefix: &str,
    ) -> Self {
        let width_pixels = 1024;
        let height_pixels = 1024;
        let tile_size = 16;
        let count_per_row = width_pixels / tile_size;
        let max_tiles = count_per_row * (height_pixels / tile_size);
        let mut atlas = ImageBuffer::new(width_pixels, height_pixels);

        log::warn!("Creating atlas from ResourceManager: {namespace}:{prefix}/");

        let mut name_to_xy_map = HashMap::new();
        let mut counter = 0;

        let ids = rm.list(namespace, prefix);
        for id in ids {
            if counter as u32 >= max_tiles {
                log::warn!("Atlas full ({} tiles), stopping", max_tiles);
                break;
            }

            // TODO: skip textures with .mcmeta files
            let bytes = match rm.get(&id) {
                Some(b) => b,
                None => continue,
            };

            let img = match image::load_from_memory(&bytes) {
                Ok(i) => i,
                Err(e) => {
                    log::warn!("Failed to decode {:?}: {}", id, e);
                    continue;
                }
            };

            if img.dimensions().0 > tile_size || img.dimensions().1 > tile_size {
                log::warn!("Image {:?} too big for tile, skipping", id);
                continue;
            }

            let x = counter as u32 % count_per_row;
            let y = counter as u32 / count_per_row;

            if let Err(e) = atlas.copy_from(&img, x * tile_size, y * tile_size) {
                log::warn!("Failed to copy {:?} into atlas: {}", id, e);
                continue;
            }

            name_to_xy_map.insert(id, uvec2(x, y));
            counter += 1;
        }

        if counter == 0 {
            log::warn!("No textures loaded, filling with default stone pattern");
            name_to_xy_map = Self::fill_default_texture(&mut atlas, tile_size);
        }

        Self {
            atlas,
            by_mip_level: None,
            tile_size,
            name_to_xy_map,
        }
    }

    fn fill_default_texture(
        atlas: &mut image::RgbaImage,
        tile_size: u32,
    ) -> HashMap<Identifier, UVec2> {
        use image::Rgba;
        let mut map = HashMap::new();
        for y in 0..tile_size {
            for x in 0..tile_size {
                let base = 128u8;
                let noise = ((x as f32 * 0.3).sin() * (y as f32 * 0.3).cos() * 20.0) as u8;
                let v = base.wrapping_add(noise);
                atlas.put_pixel(x, y, Rgba([v, v, v, 255]));
            }
        }
        map.insert(Identifier::new(&format!("{}:stone", blockworld_utils::GAME_NAME)), uvec2(0, 0));
        map
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
        assert!(x < self.width() / self.tile_size && y < self.height() / self.tile_size);
        let u1 = x * self.tile_size;
        let v1 = y * self.tile_size;
        let u2 = u1 + self.tile_size;
        let v2 = v1 + self.tile_size;
        (
            vec2(u1 as f32 / self.width() as f32, v1 as f32 / self.height() as f32),
            vec2(u2 as f32 / self.width() as f32, v2 as f32 / self.height() as f32),
        )
    }

    pub fn query_uv(&self, name: &Identifier) -> Option<(Vec2, Vec2)> {
        // Exact match first (e.g. stone → stone.png)
        if let Some(xy) = self.name_to_xy_map.get(name) {
            return Some(self.from_xy_to_uvs(*xy));
        }
        // Fallback: try {name}_top (e.g. grass_block → grass_block_top.png)
        let top_id = Identifier::new(&format!("{}_top", &**name));
        if let Some(xy) = self.name_to_xy_map.get(&top_id) {
            return Some(self.from_xy_to_uvs(*xy));
        }
        None
    }
}

impl Display for Atlas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Atlas({:?})", self.atlas.dimensions())
    }
}

#[test]
fn atlas_generation() {
    use crate::resource::{FilesystemSource, ResourceManager};
    let mut rm = ResourceManager::new();
    rm.add_source(FilesystemSource::new("../"));
    let atlas = Atlas::from_resource_manager(&rm, blockworld_utils::GAME_NAME, "textures/block");
    let count = atlas.name_to_xy_map.len();
    assert!(count > 100, "Expected >100 textures, got {}", count);
    dbg!(count);
    std::fs::create_dir_all("test_run").ok();
    atlas.save("test_run/atlas.png");
}
