//! Chunk mesh generation and rendering.
//!
//! Converts chunk block data into GPU vertex buffers.
//! Caches meshes in a `HashMap<IVec3, RenderChunk>` so chunks
//! are only rebuilt when `need_rerender` is set.

use std::collections::HashMap;

use blockworld_server::{
    block::block_face_direction::BlockFaceDirection, world::chunk_access::WorldAccess,
};
use glam::*;
use once_cell::sync::Lazy;
use wgpu::{util::DeviceExt, Device, RenderPass};

use crate::renderer::resource_manager::BLOCK_ATLAS;

use super::block_meshing::to_quad_mesh;

/// The identifier string for air blocks, computed once from `GAME_NAME`.
static AIR_STR: Lazy<String> = Lazy::new(|| format!("{}:air", blockworld_utils::GAME_NAME));

/// A GPU vertex buffer for one subchunk, plus the number of vertices to draw.
#[derive(Debug)]
pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

/// Manages chunk meshes.
///
/// `update()`: scans loaded chunks, rebuilds GPU buffers for any
/// whose `need_rerender` flag is set, then clears the flag.
/// `render()`: binds all cached vertex buffers and issues draw calls.
pub struct MeshingManager {
    /// `chunk_world_position → vertex buffer`.
    render_map: HashMap<IVec3, RenderChunk>,
}

impl MeshingManager {
    /// Rebuild meshes for chunks marked `need_rerender`.
    ///
    /// Face culling is handled by the GPU via `CullMode::Back`.
    pub fn update<T: WorldAccess>(&mut self, device: &Device, chunks: &mut T) {
        // Collect positions first to release the immutable borrow before
        // calling `clear_need_rerender` (which requires `&mut`).
        let positions: Vec<IVec3> = chunks.iter_loaded_chunks().map(|c| c.pos()).collect();

        for pos in positions {
            if chunks.need_rerender(pos) {
                let chunk = chunks.get_chunk(pos);
                let mut vertices = vec![];

                // Iterate over every block in the 16x16x16 subchunk
                for x in 0..16 {
                    for y in 0..16 {
                        for z in 0..16 {
                            let block_local = ivec3(x, y, z);
                            let block_id = chunk.get_blockid(block_local);
                            // Convert local subchunk coords to world space
                            let blockpos = pos * 16 + block_local;

                            if block_id != AIR_STR.as_str() {
                                // Look up texture UV coordinates from the atlas
                                let (a, b) = BLOCK_ATLAS
                                    .query_uv(&block_id.into())
                                    .unwrap_or((vec2(0.0, 0.0), vec2(1.0, 1.0)));
                                // Block center in world space
                                let center = blockpos.as_vec3() + vec3(0.5, 0.5, 0.5);
                                for face in BlockFaceDirection::iter() {
                                    // CPU face culling: skip faces hidden by solid neighbors
                                    if !chunks.is_air(blockpos + face.to_vec()) {
                                        continue;
                                    }
                                    let vtxs = to_quad_mesh(face, center, a, b);
                                    vertices.extend(vtxs);
                                }
                            }
                        }
                    }
                }

                let vertex_count = vertices.len() as u32;
                // Upload vertex data to GPU
                let render_chunk = RenderChunk {
                    vertex_count,
                    vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("Chunk{} Vertex Buffer", pos)),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }),
                };
                self.render_map.insert(pos, render_chunk);
                // Mark as up-to-date
                chunks.clear_need_rerender(pos);
            }
        }

        // Remove entries for chunks that have been unloaded
        self.render_map.retain(|pos, _| chunks.is_chunk_loaded(*pos));
    }

    /// Draw all cached chunk meshes.
    pub fn render<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {
        for chunk in self.render_map.values() {
            rpass.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
            rpass.draw(0..chunk.vertex_count, 0..1);
        }
    }

    pub fn new() -> Self {
        Self {
            render_map: HashMap::new(),
        }
    }
}
