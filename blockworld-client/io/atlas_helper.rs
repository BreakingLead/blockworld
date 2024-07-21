use anyhow::{anyhow, Result};
use glam::{vec2, Vec2};

/// Just a rectangular region of a texture.
#[derive(Debug, Clone, Copy)]
pub struct UV {
    aa: Vec2,
    bb: Vec2,
}

impl UV {
    pub fn new(aa: Vec2, bb: Vec2) -> Self {
        let aa = aa.clamp(vec2(0., 0.), vec2(1., 1.));
        let bb = bb.clamp(vec2(0., 0.), vec2(1., 1.));
        Self { aa, bb }
    }

    pub fn aa(&self) -> Vec2 {
        self.aa
    }

    pub fn bb(&self) -> Vec2 {
        self.bb
    }
}

/// A struct represents the schema of an atlas.
/// It doesn't contain the atlas texture by itself, it's just a helper to locate the tile uv.
pub struct Atlas {
    // atlas: Box<image::RgbImage>,
    pub tile_w: u32,
    pub tile_h: u32,
    pub image_w: u32,
    pub image_h: u32,
}

impl Atlas {
    pub fn new(path: &'static str, tile_size: u32) -> Self {
        let (image_w, image_h) = image::io::Reader::open(path)
            .unwrap()
            .into_dimensions()
            .unwrap();
        Self {
            tile_w: tile_size,
            tile_h: tile_size,
            image_w,
            image_h,
        }
    }

    pub fn get_uv_from_xy(&self, x: u32, y: u32) -> Result<UV> {
        if ((x + 1) * self.tile_w) >= self.image_w || ((x + 1) * self.tile_h) >= self.image_h {
            Err(anyhow!("Bad Input"))
        } else {
            Ok(UV::new(
                vec2(
                    (x as f32 * self.tile_w as f32) / self.image_w as f32,
                    (y as f32 * self.tile_h as f32) / self.image_h as f32,
                ),
                vec2(
                    ((x + 1) as f32 * self.tile_w as f32) / self.image_w as f32,
                    ((y + 1) as f32 * self.tile_h as f32) / self.image_h as f32,
                ),
            ))
        }
    }
}
