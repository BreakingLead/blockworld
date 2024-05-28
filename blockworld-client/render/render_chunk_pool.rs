use crate::game::chunk::Chunk;

pub struct RenderChunkPool {
    chunks: Vec<Chunk>,
}

impl RenderChunkPool {
    fn new() -> Self {
        Self { chunks: vec![] }
    }

    fn add(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
}
