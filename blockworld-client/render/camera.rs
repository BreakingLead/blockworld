use std::f32::consts::PI;

use glam::*;
use log::{debug, info};

use crate::game::player_state::PlayerState;
pub struct Camera {
    pub position: Vec3,
    up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    aspect_ratio: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    speed: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: vec3(0.0, 0.0, 5.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: PI,
            pitch: 0.0,
            aspect_ratio,
            fovy: PI / 2.0,
            znear: 0.1,
            zfar: 100.0,
            speed: 0.15,
        }
    }

    pub fn update_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
    pub fn get_gaze(&self) -> Vec3 {
        vec3(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        )
        .normalize()
    }

    pub fn build_mvp(&self) -> Mat4 {
        let gaze = self.get_gaze();
        let view = Mat4::look_to_rh(self.position, gaze, self.up);
        let projection = Mat4::perspective_rh(self.fovy, self.aspect_ratio, self.znear, self.zfar);

        projection * view
    }

    /// update camera state by 1 unit according to player_state
    pub fn update(&mut self, player_state: &PlayerState) {
        if player_state.forward {
            self.go_forward(1.0);
        }
        if player_state.backward {
            self.go_forward(-1.0);
        }
        if player_state.left {
            self.go_right(-1.0);
        }
        if player_state.right {
            self.go_right(1.0);
        }
        if player_state.ascend {
            self.go_up(1.0);
        }
        if player_state.descend {
            self.go_up(-1.0);
        }
    }

    pub fn get_forward_direction(&self) -> Vec3 {
        vec3(self.yaw.sin(), 0.0, self.yaw.cos())
    }

    fn go_forward(&mut self, step: f32) {
        let f = self.get_forward_direction();
        self.position += f * step * self.speed;
    }

    fn go_right(&mut self, step: f32) {
        let f = self.get_forward_direction();
        let r = f.cross(self.up).normalize();
        self.position += r * step * self.speed;
    }

    fn go_up(&mut self, step: f32) {
        self.position += self.up * step * self.speed;
    }

    pub fn shift(&mut self, dir: Vec3) {
        self.position += dir;
    }
}

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    matrix: [[f32; 4]; 4],
}

impl MatrixUniform {
    pub fn new() -> Self {
        Self {
            matrix: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_matrix(&mut self, camera: &Camera) {
        self.matrix = camera.build_mvp().to_cols_array_2d();
    }
}
