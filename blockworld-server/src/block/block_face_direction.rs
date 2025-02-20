use enumflags2::bitflags;
use glam::IVec3;
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BlockFaceDirection {
    /// X+ (east)
    XP = 0b000001,
    /// Y+ (up)
    YP = 0b000010,
    /// Z+ (south)
    ZP = 0b000100,
    /// X- (west)
    XN = 0b001000,
    /// Y- (down)
    YN = 0b010000,
    /// Z- (north)
    ZN = 0b100000,
}

impl BlockFaceDirection {
    pub fn iter() -> impl Iterator<Item = BlockFaceDirection> {
        [
            BlockFaceDirection::XP,
            BlockFaceDirection::YP,
            BlockFaceDirection::ZP,
            BlockFaceDirection::XN,
            BlockFaceDirection::YN,
            BlockFaceDirection::ZN,
        ]
        .iter()
        .copied()
    }
    pub fn to_vec(&self) -> IVec3 {
        match self {
            BlockFaceDirection::XP => IVec3::X,
            BlockFaceDirection::YP => IVec3::Y,
            BlockFaceDirection::ZP => IVec3::Z,
            BlockFaceDirection::XN => IVec3::NEG_X,
            BlockFaceDirection::YN => IVec3::NEG_Y,
            BlockFaceDirection::ZN => IVec3::NEG_Z,
        }
    }
}
