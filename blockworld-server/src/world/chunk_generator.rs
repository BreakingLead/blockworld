//! Terrain generation.
//!
//! Currently uses a sine-wave height function for testing.
//! Will be replaced with proper worldgen (perlin noise, biomes, etc.).

use glam::IVec3;

use super::chunk::SubChunk;
use super::chunk_map::ChunkMap;

/// Generates chunks and populates them into a `ChunkMap`.
pub struct ChunkGenerator;

impl ChunkGenerator {
    /// Fill a subchunk with blocks based on a sine-wave heightmap.
    ///
    /// Temporary test terrain — blocks below the wave are stone,
    /// everything above is air.
    pub fn generate(chunk_map: &mut ChunkMap, pos: IVec3) {
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
        chunk_map.chunks.insert(pos, sc);
        chunk_map.need_rerender.push(pos);
    }
}
