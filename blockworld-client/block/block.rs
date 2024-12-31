use blockworld_utils::{HasResourceLocation, ResourceLocation};

pub type NumberID = u32;

pub struct Block {
    pub id: ResourceLocation,
}

impl HasResourceLocation for Block {
    fn get_id(&self) -> ResourceLocation {
        self.id.clone()
    }
}

impl Block {
    pub fn new(id: ResourceLocation) -> Self {
        Self { id }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Material {
    #[default]
    Solid,
    Glass,
    Air,
}
