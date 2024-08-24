//! ```
//! package net.minecraft.client.renderer.texture;
//! class TextureAtlasSprite
//! version 1.21
//! ```

use std::{fmt::Display, rc::Rc};

use blockworld_utils::ResourceLocation;

use super::sprite_contents::SpriteContents;

pub struct TextureAtlasSprite {
    atlas_location: ResourceLocation,
    /// a reference to the big texture atlas
    contents: Rc<SpriteContents>,
    /// `final`
    x: u32,
    y: u32,
    u0: f32,
    v0: f32,
    u1: f32,
    v1: f32,
}

impl TextureAtlasSprite {
    /// `atlas_width` is `pOriginX` in the Java code
    pub fn new(
        atlas_location: ResourceLocation,
        contents: &Rc<SpriteContents>,
        x: u32,
        y: u32,
        atlas_width: u32,
        atlas_height: u32,
    ) -> Self {
        let u0 = x as f32 / atlas_width as f32;
        let u1 = (x + contents.width()) as f32 / atlas_width as f32;
        let v0 = y as f32 / atlas_height as f32;
        let v1 = (y + contents.height()) as f32 / atlas_height as f32;

        Self {
            atlas_location,
            contents: Rc::clone(contents),
            x,
            y,
            u0,
            v0,
            u1,
            v1,
        }
    }

    fn atlas_size(&self) -> f32 {
        let f = self.contents.width() as f32 / (self.u1 - self.u0);
        let f1 = self.contents.height() as f32 / (self.v1 - self.v0);
        f32::max(f1, f)
    }
}

impl Display for TextureAtlasSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TextureAtlasSprite{{contents='{}', u0={}, u1={}, v0={}, v1={}}}",
            self.contents, self.u0, self.u1, self.v0, self.v1
        )
    }
}
