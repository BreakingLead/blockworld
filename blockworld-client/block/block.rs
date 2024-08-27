use blockworld_utils::ResourceLocation;

pub type NumberID = u32;

pub trait Block: Send + Sync + 'static {
    fn texture_location(&self) -> ResourceLocation;
    fn hardness(&self) -> f32;
    fn material(&self) -> Material;
}

macro_rules! def_basic_block {
    ($name:ident, $texture:literal, $hardness:literal, $material:expr) => {
        #[derive(Eq, PartialEq, Clone, Copy)]
        pub struct $name;
        impl Block for $name {
            fn texture_location(&self) -> ResourceLocation {
                $texture.into()
            }
            fn hardness(&self) -> f32 {
                $hardness
            }
            fn material(&self) -> Material {
                $material
            }
        }
    };
}

def_basic_block!(Air, "null", 1.5, Material::Air);
def_basic_block!(Stone, "stone", 1.5, Material::Solid);
def_basic_block!(Grass, "grass", 0.6, Material::Solid);
def_basic_block!(Dirt, "dirt", 0.5, Material::Solid);

#[derive(Debug, Default, Clone, Copy)]
pub enum Material {
    #[default]
    Solid,
    Glass,
    Air,
}
