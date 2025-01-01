use std::slice::Iter;

use blockworld_utils::ResourceLocation;
use glam::IVec3;

use crate::{packet::Packet, world::chunk::SubChunk};

// readonly
// if you need to modify the chunk, you need to send a packet to the server
pub trait WorldAccess {
    // chunk coord
    fn get_chunk(&self, pos: IVec3) -> &SubChunk;
    // chunk coord
    fn is_chunk_loaded(&self, pos: IVec3) -> bool;
    // chunk coord
    fn load_chunk(&mut self, pos: IVec3);
    // chunk coord
    fn unload_chunk(&mut self, pos: IVec3);

    fn need_rerender(&self, pos: IVec3) -> bool;

    fn update(&mut self, packet: Packet);
    fn iter_loaded_chunks(&self) -> impl Iterator<Item = &SubChunk>;

    // block coord
    fn is_air(&self, pos: IVec3) -> bool;

    fn get_block(&self, pos: IVec3) -> ResourceLocation;
    fn set_block(&mut self, pos: IVec3, id: &ResourceLocation);
}
