use bevy_ecs::{schedule::Schedule, world::World};
use components::{HasView, Player};
use glam::*;
use world::disk_chunk_access::DiskChunkArray;

pub mod block;
pub mod components;
pub mod packet;
pub mod world;

pub struct Blockworld {
    chunks: DiskChunkArray,
    ecs: World,
    schedule: Schedule,
}

impl Blockworld {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let schedule = Schedule::default();
        Self {
            chunks: DiskChunkArray::new(8),
            ecs,
            schedule,
        }
    }
}
