use crate::game::{chunk_provider::ClientChunkProvider, register::RegisterTable};

use super::render_chunk::RenderChunk;

pub struct RenderArray {
    chunks: Vec<RenderChunk>,
}

impl RenderArray {
    pub fn new(
        chunk_provider: &mut ClientChunkProvider,
        device: &wgpu::Device,
        register_table: &RegisterTable,
    ) -> Self {
        let mut chunks = vec![];
        for x in -3..3 {
            for y in -3..3 {
                chunks.push(RenderChunk::new(
                    device,
                    &chunk_provider.load_chunk(x, y).unwrap(),
                    register_table,
                ));
            }
        }
        Self { chunks }
    }

    pub fn chunks(&self) -> &Vec<RenderChunk> {
        &self.chunks
    }

    pub fn update(&mut self, _chunk_provider: &mut ClientChunkProvider) {}
}
