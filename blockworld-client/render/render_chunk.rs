use glam::IVec2;
use wgpu::util::DeviceExt;

use super::vertex::Vertex;

pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

impl RenderChunk {
    pub fn new(device: &wgpu::Device, vertices: Vec<Vertex>) -> Self {
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
}
