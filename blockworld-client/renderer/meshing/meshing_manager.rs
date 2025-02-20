use blockworld_server::{
    block::block_face_direction::BlockFaceDirection,
    world::{chunk::SubChunk, chunk_access::WorldAccess, disk_chunk_access::DiskChunkArray},
};
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
    render_array: Vec<RenderChunk>,
}

impl MeshingManager {
    pub fn update<T: WorldAccess>(&mut self, device: &Device, chunks: T) {
        // loaded chunks
        for (ind, chunk) in chunks.iter_loaded_chunks().enumerate() {
            let pos = chunk.pos();
            if chunks.need_rerender(chunk.pos()) {
                let mut vertices = vec![];

                for x in 0..16 {
                    for y in 0..16 {
                        for z in 0..16 {
                            let block_id = chunk.get_blockid(pos);
                            let blockpos = pos * 16 + ivec3(x, y, z);

                            let mut cull = 0b111111 as u32;

                            if block_id != "minecraft:air" {
                                let (a, b) = BLOCK_ATLAS
                                    .query_uv(&block_id.into())
                                    .unwrap_or((vec2(0.0, 0.0), vec2(1.0, 1.0)));
                                for k in BlockFaceDirection::iter() {
                                    if !chunks.is_air(blockpos + k.to_vec()) {
                                        cull -= k as u32;
                                    }
                                }
                                for k in BlockFaceDirection::iter() {
                                    if k as u32 & cull == 0 {
                                        let vtxs = to_quad_mesh(
                                            k,
                                            vec3(pos.x as f32, pos.y as f32, pos.z as f32),
                                            a,
                                            b,
                                        );
                                        vertices.extend(vtxs);
                                    }
                                }
                            }
                        }
                    }
                }

                let render_chunk = RenderChunk {
                    vertex_count: 0,
                    vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("Chunk{} Vertex Buffer", pos)),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }),
                };
                self.render_array.push(render_chunk);
            }
        }
    }
    pub fn render<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {
        for chunk in self.render_array.iter() {
            rpass.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
            rpass.draw(0..chunk.vertex_count, 0..1);
        }
    }
    pub fn new() -> Self {
        Self {
            render_array: vec![],
        }
    }
}
