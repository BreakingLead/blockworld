//! Chunk mesh generation with background worker thread.
//!
//! Minecraft uses a ForkJoinPool of worker threads to build chunk
//! meshes on CPU, then the render thread uploads results to GPU.
//! We use a single dedicated thread + mpsc channels for the same effect.
//!
//! Flow:
//!   render thread → send chunk blocks to worker → worker tessellates
//!   → worker sends vertices back → render thread creates GPU buffer

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

use blockworld_server::{
    block::block_face_direction::BlockFaceDirection,
    world::chunk_access::WorldAccess,
};
use glam::*;
use wgpu::{util::DeviceExt, Device, RenderPass};

use crate::renderer::resource_manager::BLOCK_ATLAS;
use crate::renderer::vertex::TexturedVertex;

use super::block_meshing::to_quad_mesh;

// -- Messages ---------------------------------------------------------------

/// Sent from render thread to worker: "build mesh for this chunk".
struct MeshTask {
    pos: IVec3,
    blocks: Box<[u32; 4096]>,
}

/// Sent from worker to render thread: "here are the vertices".
struct MeshResult {
    pos: IVec3,
    vertices: Vec<TexturedVertex>,
}

// -- Worker function --------------------------------------------------------

/// Runs on the worker thread. Receives MeshTasks, tessellates block data
/// into TexturedVertex arrays, sends results back.
fn mesh_worker(task_rx: Receiver<MeshTask>, result_tx: Sender<MeshResult>) {
    while let Ok(task) = task_rx.recv() {
        let mut vertices = vec![];

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let idx = (y * 16 * 16 + z * 16 + x) as usize;
                    let block_id = task.blocks[idx];

                    if block_id != 0 {
                        // Air ID is always 0
                        let block_local = ivec3(x, y, z);
                        let blockpos = task.pos * 16 + block_local;
                        let center = blockpos.as_vec3() + vec3(0.5, 0.5, 0.5);

                        // Build identifier for UV lookup (only once per block)
                        let id_str = if let Some(r) =
                            blockworld_server::block::BLOCK_REGISTRY.number_id_to_name(block_id)
                        {
                            r
                        } else {
                            continue;
                        };

                        let (a, b) = BLOCK_ATLAS
                            .query_uv(id_str)
                            .unwrap_or((vec2(0.0, 0.0), vec2(1.0, 1.0)));

                        for face in BlockFaceDirection::iter() {
                            // Same-chunk face culling: skip faces hidden by solid neighbors
                            let neighbor_local = block_local + face.to_vec();
                            if neighbor_local.cmpge(IVec3::ZERO).all()
                                && neighbor_local.cmplt(IVec3::splat(16)).all()
                                && task.blocks[(neighbor_local.y * 16 * 16
                                    + neighbor_local.z * 16
                                    + neighbor_local.x) as usize]
                                    != 0
                            {
                                continue;
                            }
                            vertices.extend(to_quad_mesh(face, center, a, b));
                        }
                    }
                }
            }
        }

        let _ = result_tx.send(MeshResult {
            pos: task.pos,
            vertices,
        });
    }
}

// -- MeshingManager ---------------------------------------------------------

#[derive(Debug)]
pub struct RenderChunk {
    pub vertex_count: u32,
    pub vertex_buffer: wgpu::Buffer,
}

pub struct MeshingManager {
    render_map: HashMap<IVec3, RenderChunk>,
    task_tx: Sender<MeshTask>,
    result_rx: Receiver<MeshResult>,
}

impl MeshingManager {
    /// Send chunks that need remeshing to the worker thread.
    /// Collect finished meshes and upload to GPU.
    pub fn update<T: WorldAccess>(&mut self, device: &Device, chunks: &mut T) {
        let positions: Vec<IVec3> = chunks.iter_loaded_chunks().map(|c| c.pos()).collect();

        let max_per_frame = 4;
        let mut submitted = 0;

        for pos in positions {
            if chunks.need_rerender(pos) && submitted < max_per_frame {
                let chunk = chunks.get_chunk(pos);
                let task = MeshTask {
                    pos,
                    blocks: chunk.clone_blocks(),
                };
                if self.task_tx.send(task).is_ok() {
                    submitted += 1;
                    chunks.clear_need_rerender(pos);
                }
            }
        }

        // Collect completed meshes from the worker and upload to GPU
        while let Ok(result) = self.result_rx.try_recv() {
            let vertex_count = result.vertices.len() as u32;
            let render_chunk = RenderChunk {
                vertex_count,
                vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Chunk{} Vertex Buffer", result.pos)),
                    contents: bytemuck::cast_slice(&result.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            };
            self.render_map.insert(result.pos, render_chunk);
        }

        // Clean up entries for unloaded chunks
        self.render_map
            .retain(|pos, _| chunks.is_chunk_loaded(*pos));
    }

    pub fn render<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {
        for chunk in self.render_map.values() {
            rpass.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
            rpass.draw(0..chunk.vertex_count, 0..1);
        }
    }

    pub fn new() -> Self {
        let (task_tx, task_rx) = channel();
        let (result_tx, result_rx) = channel();

        std::thread::spawn(move || mesh_worker(task_rx, result_tx));

        Self {
            render_map: HashMap::new(),
            task_tx,
            result_rx,
        }
    }

    pub fn render_map_len(&self) -> usize {
        self.render_map.len()
    }
}
