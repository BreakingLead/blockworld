use std::collections::HashMap;

use anyhow::Error;
use glam::{IVec2, Vec2};

use self::block::{BlockID, BlockMeta};

pub mod block;
pub mod chunk;
pub mod player_state;

// Single Instance Mode
// ! The value of the hashmap is temporary.
pub struct RegisterTable {
    table_block: HashMap<BlockID, BlockMeta>,
}

impl RegisterTable {
    pub fn new() -> Self {
        Self {
            table_block: Default::default(),
        }
    }

    pub fn register_block(&mut self, id: BlockID, meta: BlockMeta) -> Result<(), Error> {
        if let Some(v) = self.table_block.insert(id, meta) {
            Err(Error::msg("ID Exists"))
        } else {
            Ok(())
        }
    }



    pub fn query_block(&self, id: BlockID) -> Option<&BlockMeta> {
        self.table_block.get(&id)
    }
}
