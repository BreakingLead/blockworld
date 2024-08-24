use crate::io::atlas_helper::UV;

pub type BlockID = u32;

pub trait Block {
    fn texture_name(&self) -> String;
    fn hardness(&self) -> f32;
    fn material(&self) -> Material;
}

#[derive(Eq, PartialEq)]
pub struct AirBlock;
impl Block for AirBlock {
    fn texture_name(&self) -> String {
        "block/air".to_string()
    }
    fn hardness(&self) -> f32 {
        0.0
    }
    fn material(&self) -> Material {
        Material::Air
    }
}

#[derive(Eq, PartialEq)]
pub struct StoneBlock;
impl Block for StoneBlock {
    fn texture_name(&self) -> String {
        "block/stone".to_string()
    }
    fn hardness(&self) -> f32 {
        1.5
    }
    fn material(&self) -> Material {
        Material::Solid
    }
}

#[derive(Eq, PartialEq)]
pub struct GrassBlock;
impl Block for GrassBlock {
    fn texture_name(&self) -> String {
        "block/grass".to_string()
    }
    fn hardness(&self) -> f32 {
        0.6
    }
    fn material(&self) -> Material {
        Material::Solid
    }
}

#[derive(Eq, PartialEq)]
pub struct DirtBlock;
impl Block for DirtBlock {
    fn texture_name(&self) -> String {
        "block/dirt".to_string()
    }
    fn hardness(&self) -> f32 {
        0.5
    }
    fn material(&self) -> Material {
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
