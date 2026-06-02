//! World generation using layered 2D noise.
//!
//! Produces terrain with:
//!   - stone base
//!   - 3-block dirt layer
//!   - grass surface (uses `grass_block_top` texture for all sides)
//!
//! Uses a simple hash-based value noise — no external dependencies.

use glam::IVec3;

use super::chunk::SubChunk;
use super::chunk_map::ChunkMap;

// -- Noise -----------------------------------------------------------------

/// Simple 32-bit integer hash, one multiplication and XOR-shift.
fn hash(x: i32, z: i32) -> u32 {
    let mut h = (x.wrapping_mul(374761393) ^ z.wrapping_mul(668265263)).wrapping_add(1274126177) as u32;
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^ (h >> 16)
}

/// Smooth 2D noise in [0, 1] via bilinear interpolation.
///
/// `scale` controls frequency — smaller = larger features.
fn noise(x: f32, z: f32, scale: f32) -> f32 {
    let sx = x / scale;
    let sz = z / scale;
    let ix = sx.floor() as i32;
    let iz = sz.floor() as i32;
    let fx = sx - ix as f32;
    let fz = sz - iz as f32;

    // Smoothstep for less blocky interpolation
    let u = fx * fx * (3.0 - 2.0 * fx);
    let v = fz * fz * (3.0 - 2.0 * fz);

    let n00 = hash(ix, iz) as f32 / u32::MAX as f32;
    let n10 = hash(ix + 1, iz) as f32 / u32::MAX as f32;
    let n01 = hash(ix, iz + 1) as f32 / u32::MAX as f32;
    let n11 = hash(ix + 1, iz + 1) as f32 / u32::MAX as f32;

    let nx0 = n00 + (n10 - n00) * u;
    let nx1 = n01 + (n11 - n01) * u;
    nx0 + (nx1 - nx0) * v
}

/// Fractal Brownian Motion — summed octaves for natural-looking terrain.
fn fbm(x: f32, z: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut total = 0.0;

    for _ in 0..4 {
        value += amplitude * noise(x, z, 128.0 / frequency);
        total += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    value / total
}

// -- Surface height --------------------------------------------------------

/// Base terrain height, rolling hills.
fn base_height(x: f32, z: f32) -> f32 {
    8.0 + fbm(x, z) * 16.0
}

// -- ChunkGenerator --------------------------------------------------------

pub struct ChunkGenerator;

impl ChunkGenerator {
    pub fn generate(chunk_map: &mut ChunkMap, pos: IVec3) {
        let mut sc = SubChunk::new(pos);
        let surface = 3; // dirt thickness below grass

        for x in 0..16 {
            for z in 0..16 {
                let wx = (pos.x * 16 + x) as f32;
                let wz = (pos.z * 16 + z) as f32;
                let h = base_height(wx, wz);

                for y in 0..16 {
                    let wy = (pos.y * 16 + y) as f32;
                    let local = IVec3::new(x, y, z);

                    if wy < h - surface as f32 {
                        // stone base
                        sc.set_blockid(local, &format!("{}:stone", blockworld_utils::GAME_NAME));
                    } else if wy < h {
                        // dirt layer just below surface
                        sc.set_blockid(local, &format!("{}:dirt", blockworld_utils::GAME_NAME));
                    } else if wy < h + 1.0 {
                        // grass block on top (only if there's solid below)
                        if wy >= h {
                            sc.set_blockid(local, &format!("{}:grass_block_top", blockworld_utils::GAME_NAME));
                        }
                    }
                    // else: air (already zero by default)
                }
            }
        }

        chunk_map.chunks.insert(pos, sc);
        chunk_map.need_rerender.push(pos);
    }
}
