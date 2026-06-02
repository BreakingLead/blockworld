use blockworld_server::world::chunk_generator::ChunkGenerator;
use blockworld_server::world::chunk_map::ChunkMap;
use glam::IVec3;

pub struct BlockworldClient {
    pub chunks: ChunkMap,
    view_distance: u32,
    last_center_chunk: Option<IVec3>,
}

impl BlockworldClient {
    pub fn new(view_distance: u32) -> Self {
        Self {
            chunks: ChunkMap::new(),
            view_distance,
            last_center_chunk: None,
        }
    }

    /// Load/unload chunks based on player world position.
    ///
    /// Only acts when the player crosses a chunk boundary.
    /// Newly loaded chunks get terrain generated and queued for meshing.
    pub fn update_view(&mut self, player_world_pos: glam::Vec3) {
        let center = IVec3::new(
            player_world_pos.x.div_euclid(16.0) as i32,
            0,
            player_world_pos.z.div_euclid(16.0) as i32,
        );

        if self.last_center_chunk == Some(center) {
            return;
        }
        self.last_center_chunk = Some(center);

        let vd = self.view_distance as i32;

        // Unload chunks outside view distance
        let to_unload: Vec<IVec3> = self
            .chunks
            .chunks
            .keys()
            .filter(|pos| {
                (pos.x - center.x).abs() > vd || (pos.z - center.z).abs() > vd
            })
            .cloned()
            .collect();
        for pos in to_unload {
            self.chunks.chunks.remove(&pos);
            self.chunks.need_rerender.retain(|&p| p != pos);
        }

        // Load & generate new chunks within view distance
        for dx in -vd..=vd {
            for dz in -vd..=vd {
                let chunk_pos = IVec3::new(center.x + dx, 0, center.z + dz);
                if !self.chunks.chunks.contains_key(&chunk_pos) {
                    ChunkGenerator::generate(&mut self.chunks, chunk_pos);
                }
            }
        }

        log::info!(
            "Chunks: {}/{} loaded at center {:?}",
            self.chunks.chunks.len(),
            (vd * 2 + 1).pow(2),
            center
        );
    }
}
