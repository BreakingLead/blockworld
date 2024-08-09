use crate::io::atlas_helper::UV;
use blockworld_utils::resource_location::str;

pub type BlockID = u32;

pub trait Block {
    fn texture_name() -> String;
    fn hardness() -> f32;
    fn material() -> Material;
}

pub struct Air;
impl Block for Air {
    fn texture_name() -> String {
        "block/air".to_string()
    }
    fn hardness() -> f32 {
        0.0
    }
    fn material() -> Material {
        Material::Air
    }
}

pub struct Stone;
impl Block for Stone {
    fn texture_name() -> String {
        "block/stone".to_string()
    }
    fn hardness() -> f32 {
        1.5
    }
    fn material() -> Material {
        Material::Solid
    }
}

pub struct Grass;
impl Block for Grass {
    fn texture_name() -> String {
        "block/grass".to_string()
    }
    fn hardness() -> f32 {
        0.6
    }
    fn material() -> Material {
        Material::Solid
    }
}

pub struct Dirt;
impl Block for Dirt {
    fn texture_name() -> String {
        "block/dirt".to_string()
    }
    fn hardness() -> f32 {
        0.5
    }
    fn material() -> Material {
        Material::Solid
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Material {
    #[default]
    Solid,
    Glass,
    Air,
}
