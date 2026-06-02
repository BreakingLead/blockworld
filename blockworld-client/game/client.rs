use blockworld_server::world::chunk_generator::ChunkGenerator;
use blockworld_server::world::chunk_map::ChunkMap;
use glam::IVec3;

pub struct BlockworldClient {
    pub chunks: ChunkMap,
    view_distance: u32,
    last_center_chunk: Option<IVec3>,
    /// Chunks waiting to be generated (processed 2 per frame).
    pending_generation: Vec<IVec3>,
}

impl BlockworldClient {
    pub fn new(view_distance: u32) -> Self {
        Self {
            chunks: ChunkMap::new(),
            view_distance,
            last_center_chunk: None,
            pending_generation: Vec::new(),
        }
    }

    /// Load/unload chunks based on player world position.
    ///
    /// When the player crosses a chunk boundary, this queues new chunks
    /// for generation. Actual generation happens 2 per frame in `process_queue()`.
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

        // Queue all new chunks for generation
        for dx in -vd..=vd {
            for dz in -vd..=vd {
                let chunk_pos = IVec3::new(center.x + dx, 0, center.z + dz);
                if !self.chunks.chunks.contains_key(&chunk_pos) {
                    self.pending_generation.push(chunk_pos);
                }
            }
        }

        log::info!(
            "Chunks: {}/{} ({} pending) at center {:?}",
            self.chunks.chunks.len(),
            (vd * 2 + 1).pow(2),
            self.pending_generation.len(),
            center
        );
    }

    /// Generate at most `budget` pending chunks per frame.
    pub fn process_queue(&mut self, budget: usize) {
        let count = budget.min(self.pending_generation.len());
        for chunk_pos in self.pending_generation.drain(..count) {
            ChunkGenerator::generate(&mut self.chunks, chunk_pos);
        }
    }

    pub fn pending_generation_count(&self) -> usize {
        self.pending_generation.len()
    }
}
