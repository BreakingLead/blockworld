use crate::block::*;
use crate::tileentity::TileEntity;

/// `IBlockAccess` in minecraft
trait BlockAccess {
    fn get_block() -> Block;
    fn get_tile_entity() -> TileEntity;

    ///  Returns true if the block at the specified coordinates is empty
    fn is_air_block() -> bool;
}