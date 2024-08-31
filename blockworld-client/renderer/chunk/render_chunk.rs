use std::ops::Deref;

use glam::*;
use log::info;
use wgpu::{util::DeviceExt, Device};

use crate::{
    block::BLOCK_REGISTRY,
    renderer::{block::BlockFaceDirection, resource_manager::BLOCK_ATLAS},
    world::chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE},
};

use super::super::vertex::TexturedVertex;

#[derive(Debug)]
pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

impl RenderChunk {
    pub fn new(device: &Device, chunk: &Chunk) -> Self {
        info!("New renderchunk in {:?}", chunk.pos);

        let mut vertices: Vec<TexturedVertex> = Vec::new();

        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_HEIGHT as i32 {
                for z in 0..CHUNK_SIZE as i32 {
                    let (abs_x, abs_z) = (
                        (chunk.pos.x * CHUNK_SIZE as i32 + x as i32) as f32,
                        (chunk.pos.y * CHUNK_SIZE as i32 + z as i32) as f32,
                    );
                    let block_id = chunk.get_block_id(x, y, z);

                    if block_id != "minecraft:air" {
                        let block = BLOCK_REGISTRY.get(&block_id.as_str().into());
                        if let Some(block) = block {
                            let cull_mask = chunk.exist_neighbor(x, y, z);
                            let (a, b) = BLOCK_ATLAS
                                .query_uv(&block_id.deref().into())
                                .unwrap_or((vec2(0.0, 0.0), vec2(1.0, 1.0)));

                            let mut add = |d: BlockFaceDirection| {
                                if !cull_mask.contains(d) {
                                    vertices.extend(d.to_quad_mesh(
                                        vec3(abs_x, y as f32, abs_z),
                                        a,
                                        b,
                                    ))
                                }
                            };
                            add(BlockFaceDirection::XN);
                            add(BlockFaceDirection::XP);
                            add(BlockFaceDirection::YN);
                            add(BlockFaceDirection::YP);
                            add(BlockFaceDirection::ZN);
                            add(BlockFaceDirection::ZP);
                        } else {
                            log::error!("Block {} not found in registry", block_id);
                        }
                    }
                    // info!("Block: {}", block_id);
                    // Only render queried blocks so we blocks like air won't be rendered.
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
