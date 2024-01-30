/// Describe a struct which can be used as a vertex.
pub trait AsVertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
} 

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl AsVertex for Vertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        // ULB
        position: [0.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // URB
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // URF
        position: [1.0, 1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // ULF
        position: [0.0, 1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // DLB
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // DRB
        position: [1.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // DRF
        position: [1.0, 0.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // DLF
        position: [0.0, 0.0, 1.0],
        tex_coords: [0.0, 0.0],
    },
];

// Second for test
pub const INDICES: &[u16] = &[
    // UP
    3,1,0,3,2,1,
    // DOWN
    4,6,7,4,5,6,
    // EAST
    6,1,2,6,5,1,
    // SOUTH
    7,2,3,7,6,2,
    // WEST
    4,3,0,4,7,3,
    // NORTH
    5,0,1,5,4,0,
];