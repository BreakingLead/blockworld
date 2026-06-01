use blockworld_utils::{HasIdentifier, Identifier};

pub type NumberID = u32;

pub struct Block {
    pub id: Identifier,
}

impl HasIdentifier for Block {
    fn get_id(&self) -> Identifier {
        self.id.clone()
    }
}

impl Block {
    pub fn new(id: Identifier) -> Self {
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
