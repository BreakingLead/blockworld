use std::mem::size_of;

use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TexturedVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl TexturedVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![10 => Float32x3, 11 => Float32x2];

    pub fn new(pos: Vec3, uv: Vec2) -> Self {
        Self {
            position: pos.to_array(),
            uv: uv.to_array(),
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<TexturedVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
