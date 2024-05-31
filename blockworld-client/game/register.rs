use std::collections::HashMap;

use anyhow::Error;

use block::{BlockID, BlockMeta};

use super::block;

#[derive(Debug)]
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
