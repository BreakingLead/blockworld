use bevy_ecs::{schedule::Schedule, system::Res, world::World};
use bevy_input::{keyboard::KeyCode, ButtonInput};

struct BlockworldClient {
    ecs: World,
    schedule: Schedule,
}

impl BlockworldClient {
    fn new() -> Self {
        let ecs = World::default();
        let schedule = Schedule::default();
        Self { ecs, schedule }
    }
}
