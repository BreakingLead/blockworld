use bevy_ecs::{schedule::Schedule, world::World};
use blockworld_server::world::disk_chunk_access::DiskChunkArray;

pub struct BlockworldClient {
    ecs: World,
    schedule: Schedule,

    chunks: DiskChunkArray,
}

impl BlockworldClient {
    pub fn new() -> Self {
        let ecs = World::default();
        let schedule = Schedule::default();
        Self {
            ecs,
            schedule,
            chunks: DiskChunkArray::new(4),
        }
    }
}
