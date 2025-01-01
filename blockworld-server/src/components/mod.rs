use bevy_ecs::component::Component;
use glam::*;

#[derive(Component)]
pub struct HasView {
    pub position: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub aspect_ratio: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Player;
