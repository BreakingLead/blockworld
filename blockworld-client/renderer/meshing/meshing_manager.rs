use blockworld_server::world::{chunk::SubChunk, chunk_access::WorldAccess};
use wgpu::RenderPass;

#[derive(Debug)]
pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

pub struct MeshingManager {
    render_array: Vec<RenderChunk>,
}

impl MeshingManager {
    pub fn update<T: WorldAccess>(&mut self, chunks: T) {
        for i in chunks.iter_loaded_chunks() {
            if chunks.need_rerender(i.pos()) {
                continue;
            }
        }
    }
    pub fn render<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {}
}
