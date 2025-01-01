//! net/minecraft/client/multiplayer/ClientChunkProvider.java

use std::collections::HashMap;

use glam::*;

use crate::packet::Packet;

use super::{chunk::SubChunk, chunk_access::WorldAccess};

fn world_blockpos_to_chunkpos(pos: IVec3) -> (IVec3, IVec3) {
    let x = pos.x / 16;
    let y = pos.y / 16;
    let z = pos.z / 16;
    let sub_x = pos.x.rem_euclid(16);
    let sub_y = pos.y.rem_euclid(16);
    let sub_z = pos.z.rem_euclid(16);
    (IVec3::new(x, y, z), IVec3::new(sub_x, sub_y, sub_z))
}

/// The place which holds all loaded chunks
pub struct DiskChunkAccess {
    pub chunks: HashMap<IVec3, SubChunk>,
    /// Set this with the view distance
    view_distance: u32,
    /// Coord of the center chunk which is where we are in
    center: IVec3,
    /// The count of loaded chunks
    loaded: u32,

    pub need_rerender: Vec<IVec3>,
}

impl DiskChunkAccess {
    /// Create a default chunk array with view distance.
    ///
    /// Look up for settings to get the view distance.
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

    /// Check if the chunk [x, z] is in the view distance.
    pub fn in_view(&self, chunk_x: i32, chunk_z: i32) -> bool {
        (chunk_x - self.center.x).abs() <= self.view_distance as i32
            && (chunk_z - self.center.y).abs() <= self.view_distance as i32
    }

    fn generator(&mut self, pos: IVec3) -> SubChunk {
        let mut sc = SubChunk::new(pos);
        for x in 0..=15 {
            for y in 0..=15 {
                for z in 0..=15 {
                    let [wx, wy, wz] = (IVec3::new(x, y, z) + pos * 16).to_array();
                    if (wy as f32) < (wy as f32).sin() * 30.0 {
                        sc.set_blockid(pos, "minecraft:stone".into());
                    }
                }
            }
        }
        sc
    }

    pub fn recenter(&mut self, pos: IVec3) {
        self.center = pos;
    }
}

impl WorldAccess for DiskChunkAccess {
    fn is_chunk_loaded(&self, pos: IVec3) -> bool {
        self.chunks.contains_key(&pos)
    }

    fn get_chunk(&self, pos: IVec3) -> &SubChunk {
        self.chunks.get(&pos).unwrap()
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
            // TODO: serialize chunk to disk
            self.loaded -= 1;
        } else {
            log::error!("Tried to unload non-existent chunk: {}", pos);
        }
    }

    fn iter_loaded_chunks(&self) -> impl Iterator<Item = &SubChunk> {
        self.chunks.values()
    }

    fn update(&mut self, packet: Packet) {
        if let Packet::BlockUpdate(pos, id) = packet {
            if self.is_chunk_loaded(pos) {
                self.need_rerender.push(pos);
                let chunk = self.chunks.get_mut(&pos).unwrap();
                chunk.set_blockid(pos, &id);
            }
        }
    }

    fn is_air(&self, pos: IVec3) -> bool {
        self.get_block(pos) == "minecraft:air".into()
    }

    fn get_block(&self, pos: IVec3) -> blockworld_utils::ResourceLocation {
        let (a, b) = world_blockpos_to_chunkpos(pos);
        self.get_chunk(a).get_blockid(b).into()
    }

    fn set_block(&mut self, pos: IVec3, id: &blockworld_utils::ResourceLocation) {
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
}
