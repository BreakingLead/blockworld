//! Utils for creating bind group, bind group layout and the uniform

use glam::Mat4;
use wgpu::{util::DeviceExt, Queue};

pub trait ToBytes: Copy + Clone + bytemuck::Pod {
    fn to_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

/// T refers to the uniform type
pub struct Uniform<T>
where
    T: ToBytes,
{
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub binding: u32,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawMat4(pub [[f32; 4]; 4]);
impl ToBytes for RawMat4 {}

impl From<Mat4> for RawMat4 {
    fn from(mat: Mat4) -> Self {
        Self(mat.to_cols_array_2d())
    }
}

impl<T: ToBytes> Uniform<T> {
    pub fn update(&mut self, queue: &Queue, new_value: T) {
        // Upload the new uniform buffer to the GPU
        queue.write_buffer(&self.buffer, 0, new_value.to_bytes());
    }

    /// Create a new uniform with the given device, uniform value, binding number and label
    ///
    /// binding number corresponds to the shader's @binding(x)
    pub fn new(device: &wgpu::Device, uniform: T, binding: u32, label: Option<&str>) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                // @binding(x)
                binding,
                resource: buffer.as_entire_binding(),
            }],
            label,
        });

        Self {
            buffer,
            layout,
            bind_group,
            binding,
            _phantom: std::marker::PhantomData,
        }
    }
}

// impl<T> Deref for Uniform<T>
// where
//     T: ToBytes,
// {
//     type Target = [u8];

//     fn deref(&self) -> &Self::Target {
//         self.uniform.to_bytes()
//     }
// }
