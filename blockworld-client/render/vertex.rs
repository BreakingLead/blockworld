#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
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

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    // RUB
    Vertex {
        position: [1.00000, 1.00000, 000000.0],
        uv: [0.0, 0.0],
    },
    // RDB
    Vertex {
        position: [1.00000, 0.00000, 000000.0],
        uv: [1.0, 0.0],
    },
    // RUF 
    Vertex {
        position: [1.00000, 1.00000, 1.00000],
        uv: [0.0, 1.0],
    },
    // RDF
    Vertex {
        position: [1.00000, 000000.0, 1.00000],
        uv: [1.0, 1.0],
    },
    // LUB
    Vertex {
        position: [0.0, 1.00000, 00.00000],
        uv: [0.0, 0.0],
    },
    // LDB
    Vertex {
        position: [0.00000, 000000.0, 0.00000],
        uv: [1.0, 0.0],
    },
    // LUF
    Vertex {
        position: [0.00000, 1.00000, 1.00000],
        uv: [0.0, 1.0],
    },
    // LDF
    Vertex {
        position: [0.00000, 000000.0, 1.00000],
        uv: [1.0, 1.0],
    },
];

#[rustfmt::skip]
pub const INDICES: &[u32] = &[
4, 2, 0, 2, 7, 3, 6, 5, 7, 1, 7, 5, 0, 3, 1, 4, 1, 5, 4, 6, 2, 2, 6, 7, 6, 4, 5, 1, 3, 7, 0, 2, 3, 4, 0, 1
];
