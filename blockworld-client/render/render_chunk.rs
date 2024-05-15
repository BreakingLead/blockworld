use glam::{vec2, vec3, IVec2};
use log::info;
use wgpu::{util::DeviceExt, Device};

use crate::{
    game::{
        block,
        chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE},
        RegisterTable,
    },
    io::atlas_helper::AtlasMeta,
};

use super::{draw::State, render_block::*, vertex::Vertex};

#[derive(Debug)]
pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

impl RenderChunk {
    pub fn new(
        device: &Device,
        chunk: &Chunk,
        register_table: &RegisterTable,
        atlas_meta: &AtlasMeta,
    ) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in (0..CHUNK_SIZE) {
            for y in (0..CHUNK_HEIGHT) {
                for z in (0..CHUNK_SIZE) {
                    let (ax, az) = (
                        (chunk.coord.x * CHUNK_SIZE as i32 + x as i32) as f32,
                        (chunk.coord.y * CHUNK_SIZE as i32 + z as i32) as f32,
                    );
                    let block_id = chunk.blocks[Chunk::index_from_xyz(x, y, z)].id;
                    // info!("Block: {}", block_id);
                    // Only render queried blocks so we blocks like air won't be rendered.
                    if let Some(meta) = register_table.query_block(block_id) {
                        push_face_mesh(
                            &mut vertices,
                            XN,
                            vec3(ax, y as f32, az),
                            meta.atlas_coord[0],
                        );
                        push_face_mesh(
                            &mut vertices,
                            XP,
                            vec3(ax, y as f32, az),
                            meta.atlas_coord[1],
                        );
                        push_face_mesh(
                            &mut vertices,
                            YN,
                            vec3(ax, y as f32, az),
                            meta.atlas_coord[2],
                        );
                        push_face_mesh(
                            &mut vertices,
                            YP,
                            vec3(ax, y as f32, az),
                            meta.atlas_coord[3],
                        );
                        push_face_mesh(
                            &mut vertices,
                            ZN,
                            vec3(ax, y as f32, az),
                            meta.atlas_coord[4],
                        );
                        push_face_mesh(
                            &mut vertices,
                            ZP,
                            vec3(ax, y as f32, az),
                            meta.atlas_coord[5],
                        );
                    }
                }
            }
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        RenderChunk {
            vertex_count: vertices.len() as u32,
            vertex_buffer,
        }
    }
    // pub fn update_mesh(&mut self, device: &wgpu::Device)
}
