use blockworld_utils::ResourceLocation;
use glam::{IVec3, Vec3};

pub enum Packet {
    BlockUpdate(IVec3, String),
    MoveTo(Vec3),
    // no use
    Pass,
}
