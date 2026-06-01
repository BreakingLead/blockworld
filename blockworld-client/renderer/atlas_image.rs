//! ```
//! package net.minecraft.client.renderer.texture
//! class SpriteContents
//! version 1.21
//! ```

use std::{collections::HashMap, fmt::Display, path::Path};

use blockworld_utils::ResourceLocation;
use glam::{ivec2, uvec2, vec2, IVec2, UVec2, Vec2};
use image::{GenericImage, GenericImageView, ImageBuffer};

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
        let width_pixels = 1024;
        let height_pixels = 1024;
        let tile_size = 16;
        let count_per_row = width_pixels / tile_size;
        let max_tiles = count_per_row * (height_pixels / tile_size);
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

        if let Ok(dir) = assets_path.as_ref().read_dir() {
            for entry in dir {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let path = entry.path();
                let has_mcmeta = {
                    let mcmeta = format!("{}.mcmeta", path.display());
                    std::path::Path::new(&mcmeta).exists()
                };
                if path.is_file()
                    && path.extension().map_or(false, |e| e == "png")
                    && !has_mcmeta
                {
                    if counter as u32 >= max_tiles {
                        log::warn!("Atlas full ({} tiles), skipping remaining textures", max_tiles);
                        break;
                    }
                    let x = counter as u32 % count_per_row;
                    let y = counter as u32 / count_per_row;
                    let img = match image::open(&path) {
                        Ok(i) => i,
                        Err(e) => {
                            log::warn!("Failed to open image {}: {}", path.display(), e);
                            continue;
                        }
                    };

                    if img.dimensions().0 > tile_size || img.dimensions().1 > tile_size {
                        // TODO: read meta, then reimplement this
                        log::warn!(
                            "Image {} is too big for the tile size, ignoring",
                            path.display()
                        );
                        continue;
                    }

                    if let Err(e) = atlas.copy_from(&img, x * tile_size, y * tile_size) {
                        log::warn!("Failed to copy image {} into atlas: {}", path.display(), e);
                        continue;
                    }

                    if let Some(item_name) = path.file_stem() {
                        let r = ResourceLocation::new(
                            format!("minecraft:{}", item_name.to_str().unwrap_or("unknown"))
                                .as_str(),
                        );
                        name_to_xy_map.insert(r, uvec2(x, y));
                    }

                    counter += 1;
                }
            }
        } else {
            log::warn!(
                "Texture atlas directory not found: {:?}.",
                assets_path.as_ref()
            );
        }

        if counter == 0 {
            log::warn!("No textures loaded, filling with default patterns");
            name_to_xy_map = Self::fill_default_texture(&mut atlas, tile_size);
            counter = 1;
        }

        Self {
            atlas,
            by_mip_level: None,
            tile_size,
            name_to_xy_map,
        }
    }

    fn fill_default_texture(atlas: &mut image::RgbaImage, tile_size: u32) -> HashMap<ResourceLocation, UVec2> {
        use image::Rgba;
        let mut map = HashMap::new();
        // Create a simple stone-like texture for the default tile
        for y in 0..tile_size {
            for x in 0..tile_size {
                let base = 128u8;
                let noise = ((x as f32 * 0.3).sin() * (y as f32 * 0.3).cos() * 20.0) as u8;
                let v = base.wrapping_add(noise);
                atlas.put_pixel(x, y, Rgba([v, v, v, 255]));
            }
        }
        map.insert(
            ResourceLocation::new("minecraft:stone"),
            uvec2(0, 0),
        );
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
    // Tests run from crate root (blockworld-client/), assets are at workspace root
    let atlas = Atlas::new(Path::new("../assets/minecraft/textures/block"));
    let count = atlas.name_to_xy_map.len();
    assert!(count > 100, "Expected >100 textures, got {}", count);
    dbg!(count);
    std::fs::create_dir_all("run").ok();
    atlas.save("run/atlas.png");
}
