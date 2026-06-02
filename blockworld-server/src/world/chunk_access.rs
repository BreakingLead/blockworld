//! Abstract world access interface.
//!
//! Equivalent to Minecraft's `IBlockReader` / `LevelReader`.
//! The trait separates "what you can query" from "how chunks are stored",
//! allowing different backends: in-memory HashMap, disk, network-synced.

use blockworld_utils::Identifier;
use glam::IVec3;

use crate::{packet::Packet, world::chunk::SubChunk};

/// Read-only world access with write operations gated behind `&mut self`.
///
/// Coordinates come in two flavors:
///   - **chunk coordinates**: which 16³ subchunk (e.g. `(0, 0, 0)` = chunk at origin)
///   - **block coordinates**: absolute world position
pub trait WorldAccess {
    // -- chunk-level operations (coordinates in chunk space) --
    fn get_chunk(&self, pos: IVec3) -> &SubChunk;
    fn is_chunk_loaded(&self, pos: IVec3) -> bool;
    fn load_chunk(&mut self, pos: IVec3);
    fn unload_chunk(&mut self, pos: IVec3);

    /// Iterate over all currently loaded chunks.
    fn iter_loaded_chunks(&self) -> impl Iterator<Item = &SubChunk>;

    /// Apply a network packet (block update, etc.).
    fn update(&mut self, packet: Packet);

    // -- block-level operations (coordinates in world space) --
    fn is_air(&self, pos: IVec3) -> bool;
    fn get_block(&self, pos: IVec3) -> Identifier;
    fn set_block(&mut self, pos: IVec3, id: &Identifier);

    // -- mesh invalidation --
    /// Whether this chunk's render mesh needs regeneration.
    fn need_rerender(&self, pos: IVec3) -> bool;
    /// Called by the meshing system after rebuilding the chunk.
    fn clear_need_rerender(&mut self, pos: IVec3);
}
