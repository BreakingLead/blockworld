#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![10 => Float32x3, 11 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.500000, 0.500000, -0.500000],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [0.500000, -0.500000, -0.500000],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [0.500000, 0.500000, 0.500000],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [0.500000, -0.500000, 0.500000],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-0.500000, 0.500000, -0.500000],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-0.500000, -0.500000, -0.500000],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-0.500000, 0.500000, 0.500000],
        uv: [0.5, 0.5],
    },
    Vertex {
        position: [-0.500000, -0.500000, 0.500000],
        uv: [0.0, 0.0],
    },
];

#[rustfmt::skip]
pub const INDICES: &[u32] = &[
    5,3,1,3,8,4,
    7,6,8,2,8,6,
    1,4,2,5,2,6,
    5,7,3,3,7,8,
    7,5,6,2,4,8,
    1,3,4,5,1,2,
];
