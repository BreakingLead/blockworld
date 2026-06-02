//! In-memory chunk storage with view-distance culling.
//!
//! Equivalent to Minecraft's `ClientChunkProvider` / `ClientChunkCache`.
//! Stores chunks in a `HashMap<IVec3, SubChunk>`, generates test terrain
//! via a sine-wave height function, and tracks which chunks need mesh regeneration.

use std::collections::HashMap;

use glam::*;

use crate::packet::Packet;

use super::{chunk::SubChunk, chunk_access::WorldAccess};

// -- Coordinate helpers ----------------------------------------------------

/// Decompose a world-space block position into `(chunk_coord, local_coord)`.
///
/// `chunk_coord` = `floor(pos / 16)` — which 16³ subchunk contains this block.
/// `local_coord` = `pos mod 16` — offset within that subchunk (0..15).
///
/// Uses `rem_euclid` so negative coordinates wrap correctly
/// (e.g. block at `-1` → local `15` in chunk `-1`).
fn world_blockpos_to_chunkpos(pos: IVec3) -> (IVec3, IVec3) {
    let x = pos.x / 16;
    let y = pos.y / 16;
    let z = pos.z / 16;
    let sub_x = pos.x.rem_euclid(16);
    let sub_y = pos.y.rem_euclid(16);
    let sub_z = pos.z.rem_euclid(16);
    (IVec3::new(x, y, z), IVec3::new(sub_x, sub_y, sub_z))
}

// -- DiskChunkArray --------------------------------------------------------

/// The in-memory chunk cache.
///
/// Holds all currently loaded subchunks plus metadata for view-distance
/// management and render invalidation.
pub struct DiskChunkArray {
    pub chunks: HashMap<IVec3, SubChunk>,
    /// Maximum distance (in chunks) the player can see.
    view_distance: u32,
    /// Chunk coordinate the player is currently centered on.
    center: IVec3,
    /// Number of currently loaded chunks.
    loaded: u32,
    /// List of chunk coordinates whose render meshes are stale.
    /// The `MeshingManager` reads this, rebuilds the GPU buffers,
    /// then calls `clear_need_rerender`.
    pub need_rerender: Vec<IVec3>,
}

impl DiskChunkArray {
    /// Create an empty chunk cache with the given view distance (in chunks).
    pub fn new(view_distance: u32) -> Self {
        let side_length = (view_distance * 2 + 1) as usize;
        let mut chunks = HashMap::with_capacity(side_length * side_length * 16);
        Self {
            view_distance,
            chunks,
            center: IVec3::ZERO,
            loaded: 0,
            need_rerender: Vec::new(),
        }
    }

    /// Check if the chunk at `(chunk_x, chunk_z)` is within the player's view.
    pub fn in_view(&self, chunk_x: i32, chunk_z: i32) -> bool {
        (chunk_x - self.center.x).abs() <= self.view_distance as i32
            && (chunk_z - self.center.y).abs() <= self.view_distance as i32
    }

    /// Generate test terrain for a chunk using a sine-wave heightmap.
    ///
    /// Temporary — will be replaced with proper worldgen.
    /// Fills blocks below the height with stone, leaves the rest air.
    pub fn generate_chunk(&mut self, pos: IVec3) {
        let mut sc = SubChunk::new(pos);
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let world_y = (pos.y * 16 + y) as f32;
                    let height = 8.0
                        + (pos.x as f32 * 0.3).sin() * 3.0
                        + (pos.z as f32 * 0.3).cos() * 3.0;
                    if world_y < height {
                        sc.set_blockid(
                            IVec3::new(x, y, z),
                            &format!("{}:stone", blockworld_utils::GAME_NAME),
                        );
                    }
                }
            }
        }
        self.chunks.insert(pos, sc);
        self.need_rerender.push(pos);
    }

    /// Move the view center (called when the player crosses a chunk boundary).
    pub fn recenter(&mut self, pos: IVec3) {
        self.center = pos;
    }
}

// -- WorldAccess implementation --------------------------------------------

impl WorldAccess for DiskChunkArray {
    fn is_chunk_loaded(&self, pos: IVec3) -> bool {
        self.chunks.contains_key(&pos)
    }

    fn get_chunk(&self, pos: IVec3) -> &SubChunk {
        self.chunks
            .get(&pos)
            .expect("Chunk not loaded - call is_chunk_loaded() first")
    }

    fn load_chunk(&mut self, pos: IVec3) {
        if self.chunks.get(&pos).is_none() {
            self.loaded += 1;
            self.need_rerender.push(pos);
            let sc = SubChunk::new(pos);
            self.chunks.insert(pos, sc);
        }
    }

    fn unload_chunk(&mut self, pos: IVec3) {
        self.need_rerender
            .swap_remove(self.need_rerender.iter().position(|x| *x == pos).unwrap());

        if let Some(mut chunk) = self.chunks.remove(&pos) {
            // TODO: serialize chunk to disk before dropping
            self.loaded -= 1;
        } else {
            log::error!("Tried to unload non-existent chunk: {}", pos);
        }
    }

    fn iter_loaded_chunks(&self) -> impl Iterator<Item = &SubChunk> {
        self.chunks.values()
    }

    /// Handle an incoming network packet.
    /// Currently only `BlockUpdate` — updates the block and flags the chunk for remesh.
    fn update(&mut self, packet: Packet) {
        if let Packet::BlockUpdate(pos, id) = packet {
            if self.is_chunk_loaded(pos) {
                self.need_rerender.push(pos);
                let chunk = self.chunks.get_mut(&pos).unwrap();
                chunk.set_blockid(pos, &id);
            }
        }
    }

    /// A position is "air" if the chunk isn't loaded (treat unloaded as empty)
    /// or if the block is the air identifier.
    fn is_air(&self, pos: IVec3) -> bool {
        let (chunk_pos, _) = world_blockpos_to_chunkpos(pos);
        if !self.is_chunk_loaded(chunk_pos) {
            return true;
        }
        self.get_block(pos)
            == blockworld_utils::Identifier::new(&format!(
                "{}:air",
                blockworld_utils::GAME_NAME
            ))
    }

    fn get_block(&self, pos: IVec3) -> blockworld_utils::Identifier {
        let (chunk_pos, block_pos) = world_blockpos_to_chunkpos(pos);
        self.get_chunk(chunk_pos).get_blockid(block_pos).into()
    }

    fn set_block(&mut self, pos: IVec3, id: &blockworld_utils::Identifier) {
        let (a, b) = world_blockpos_to_chunkpos(pos);
        if self.is_chunk_loaded(a) {
            self.chunks
                .get_mut(&a)
                .unwrap()
                .set_blockid(b, &id.to_string());
            self.need_rerender.push(a);
        }
    }

    fn need_rerender(&self, pos: IVec3) -> bool {
        self.need_rerender.contains(&pos)
    }

    /// Remove a chunk from the remesh queue after the MeshingManager rebuilds it.
    fn clear_need_rerender(&mut self, pos: IVec3) {
        self.need_rerender.retain(|&p| p != pos);
    }
}
