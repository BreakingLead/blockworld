//! Utils for creating bind group, bind group layout and the uniform

use glam::Mat4;
use wgpu::util::DeviceExt;

/// T refers to the uniform type
pub struct Uniform<T>
where
    T: Copy + Clone + bytemuck::Pod + bytemuck::Zeroable + bytemuck::NoUninit,
{
    pub uniform: Box<T>,
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub binding: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawMat4(pub [[f32; 4]; 4]);

impl From<Mat4> for RawMat4 {
    fn from(mat: Mat4) -> Self {
        Self(mat.to_cols_array_2d())
    }
}

impl<T: bytemuck::Pod> Uniform<T> {
    /// Create a new uniform with the given device, uniform value, binding number and label
    pub fn new(device: &wgpu::Device, uniform: T, binding: u32, label: Option<&str>) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            // ? Should add a " layout" after label
            label,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                // same as above binding number
                binding,
                resource: buffer.as_entire_binding(),
            }],
            label,
        });

        let uniform = Box::new(uniform);

        Self {
            uniform,
            buffer,
            layout,
            bind_group,
            binding,
        }
    }
}
