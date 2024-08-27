mod init_helpers;
mod render_state;
pub mod texture;
pub mod uniform;
pub mod window_init;

pub mod pipeline;

pub trait HasBindGroupLayout {
    fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout;
}
