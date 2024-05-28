use anyhow::Error;
use glam::{vec2, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct AtlasCoordinate {
    aa: Vec2,
    bb: Vec2,
}

impl AtlasCoordinate {
    pub fn new(aa: Vec2, bb: Vec2) -> Self {
        aa.clamp(vec2(0., 0.), vec2(1., 1.));
        bb.clamp(vec2(0., 0.), vec2(1., 1.));
        Self { aa, bb }
    }

    pub fn aa(&self) -> Vec2 {
        self.aa
    }

    pub fn bb(&self) -> Vec2 {
        self.bb
    }
}

// Helper to find correct AABB of a texture
#[derive(Debug)]
pub struct AtlasMeta {
    pub tile_w: u32,
    pub tile_h: u32,
    pub image_w: u32,
    pub image_h: u32,
}

impl AtlasMeta {
    pub fn get(&self, x: u32, y: u32) -> Result<AtlasCoordinate, Error> {
        if ((x + 1) * self.tile_w) >= self.image_w || ((x + 1) * self.tile_h) >= self.image_h {
            Err(Error::msg("Bad input"))
        } else {
            let r = Ok(AtlasCoordinate::new(
                vec2(
                    (x as f32 * self.tile_w as f32) / self.image_w as f32,
                    (y as f32 * self.tile_h as f32) / self.image_h as f32,
                ),
                vec2(
                    ((x + 1) as f32 * self.tile_w as f32) / self.image_w as f32,
                    ((y + 1) as f32 * self.tile_h as f32) / self.image_h as f32,
                ),
            ));
            dbg!(&r);
            r
        }
    }
}
