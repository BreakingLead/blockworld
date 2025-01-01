use enumflags2::bitflags;
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
