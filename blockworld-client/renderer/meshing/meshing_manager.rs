use std::collections::HashMap;

use blockworld_server::world::chunk_access::WorldAccess;
use glam::*;
use wgpu::{util::DeviceExt, Device, RenderPass};

use crate::renderer::resource_manager::BLOCK_ATLAS;

use super::block_meshing::to_quad_mesh;

#[derive(Debug)]
pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

pub struct MeshingManager {
    render_map: HashMap<IVec3, RenderChunk>,
}

impl MeshingManager {
    pub fn update<T: WorldAccess>(&mut self, device: &Device, chunks: &mut T) {
        let positions: Vec<IVec3> = chunks.iter_loaded_chunks().map(|c| c.pos()).collect();

        for pos in positions {
            if chunks.need_rerender(pos) {
                let chunk = chunks.get_chunk(pos);
                let mut vertices = vec![];

                for x in 0..16 {
                    for y in 0..16 {
                        for z in 0..16 {
                            let block_local = ivec3(x, y, z);
                            let block_id = chunk.get_blockid(block_local);
                            let blockpos = pos * 16 + block_local;

                            if block_id != "minecraft:air" {
                                let (a, b) = BLOCK_ATLAS
                                    .query_uv(&block_id.into())
                                    .unwrap_or((vec2(0.0, 0.0), vec2(1.0, 1.0)));
                                let center = blockpos.as_vec3() + vec3(0.5, 0.5, 0.5);
                                for face in blockworld_server::block::block_face_direction::BlockFaceDirection::iter() {
                                    let vtxs = to_quad_mesh(face, center, a, b);
                                    vertices.extend(vtxs);
                                }
                            }
                        }
                    }
                }

                let vertex_count = vertices.len() as u32;
                let render_chunk = RenderChunk {
                    vertex_count,
                    vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("Chunk{} Vertex Buffer", pos)),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }),
                };
                self.render_map.insert(pos, render_chunk);
                chunks.clear_need_rerender(pos);
            }
        }
    }

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
