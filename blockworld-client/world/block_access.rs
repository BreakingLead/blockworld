use crate::block::*;

/// `IBlockAccess` in minecraft
trait BlockAccess {
    fn get_block(&self) -> &'static dyn Block;
    // fn get_tile_entity(&self) -> TileEntity;

    ///  Returns true if the block at the specified coordinates is empty
    fn is_air_block(&self) -> bool;
}
