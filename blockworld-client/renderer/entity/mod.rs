use glam::Vec3;

pub struct Player {
    pub position: Vec3,
    pub rotation: Vec3, // Yaw, Pitch, Roll
}

impl Player {
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        Self { position, rotation }
    }

    pub fn update(&mut self) {}
}
