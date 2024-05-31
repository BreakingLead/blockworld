use super::render_block::*;
use glam::*;
use wgpu::{util::DeviceExt, Device};

use crate::game::{chunk::*, register::RegisterTable};

use super::vertex::Vertex;

#[derive(Debug)]
pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

impl RenderChunk {
    pub fn new(device: &Device, chunk: &Chunk, register_table: &RegisterTable) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_HEIGHT as i32 {
                for z in 0..CHUNK_SIZE as i32 {
                    let (abs_x, abs_z) = (
                        (chunk.pos.x * CHUNK_SIZE as i32 + x as i32) as f32,
                        (chunk.pos.z * CHUNK_SIZE as i32 + z as i32) as f32,
                    );
                    let block_id = chunk.blocks[Chunk::index(x, y, z)].id;
                    // info!("Block: {}", block_id);
                    // Only render queried blocks so we blocks like air won't be rendered.

                    if let Some(meta) = register_table.query_block(block_id) {
                        let neighbors = chunk.detect_block_neighbors(x, y, z);
                        // Exist block in that way
                        if (neighbors & XN_B) == 0 {
                            push_face_mesh(
                                &mut vertices,
                                XN,
                                vec3(abs_x, y as f32, abs_z),
                                meta.atlas_coord[0],
                            );
                        }
                        if (neighbors & XP_B) == 0 {
                            push_face_mesh(
                                &mut vertices,
                                XP,
                                vec3(abs_x, y as f32, abs_z),
                                meta.atlas_coord[1],
                            );
                        }
                        if (neighbors & YN_B) == 0 {
                            push_face_mesh(
                                &mut vertices,
                                YN,
                                vec3(abs_x, y as f32, abs_z),
                                meta.atlas_coord[2],
                            );
                        }
                        if (neighbors & YP_B) == 0 {
                            push_face_mesh(
                                &mut vertices,
                                YP,
                                vec3(abs_x, y as f32, abs_z),
                                meta.atlas_coord[3],
                            );
                        }
                        if (neighbors & ZN_B) == 0 {
                            push_face_mesh(
                                &mut vertices,
                                ZN,
                                vec3(abs_x, y as f32, abs_z),
                                meta.atlas_coord[4],
                            );
                        }
                        if (neighbors & ZP_B) == 0 {
                            push_face_mesh(
                                &mut vertices,
                                ZP,
                                vec3(abs_x, y as f32, abs_z),
                                meta.atlas_coord[5],
                            );
                        }
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

    // Preserved
    pub fn update_mesh(&mut self) {
        let _ = 1;
    }
}
