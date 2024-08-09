use crate::block::*;
use anyhow::Error;
use block::{BlockID, BlockMeta};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockRegisterTable {
    table_block: HashMap<BlockID, BlockMeta>,
}

impl RegisterTable {
    pub fn new() -> Self {
        Self {
            table_block: Default::default(),
        }
    }

    pub fn register_block(&mut self, id: BlockID, meta: BlockMeta) -> Result<(), Error> {
        if let Some(_meta) = self.table_block.insert(id, meta) {
            Err(Error::msg("ID Exists"))
        } else {
            Ok(())
        }
    }

    pub fn query_block(&self, id: BlockID) -> Option<&BlockMeta> {
        self.table_block.get(&id)
    }
}
