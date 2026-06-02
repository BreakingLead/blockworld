//! GPU vertex format for textured block faces.
//!
//! Uses WGSL locations 10 (position) and 11 (uv) to avoid
//! conflicts with future vertex formats (e.g. location 0 for
//! UI vertices, location 20 for particles, etc.).

use std::mem::size_of;

use glam::{Vec2, Vec3};

/// A single vertex with position and texture coordinates.
///
/// `Pod` + `Zeroable` so it can be uploaded directly via `bytemuck`.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TexturedVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl TexturedVertex {
    /// Vertex attribute layout: location 10 = vec3, location 11 = vec2.
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![10 => Float32x3, 11 => Float32x2];

    pub fn new(pos: Vec3, uv: Vec2) -> Self {
        Self {
            position: pos.to_array(),
            uv: uv.to_array(),
        }
    }

    /// Returns the `VertexBufferLayout` describing this vertex format.
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<TexturedVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
