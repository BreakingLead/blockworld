use std::f32::consts::PI;

use glam::*;

#[derive(Debug)]
pub struct Camera {
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

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: vec3(0.0, 10.0, 5.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            aspect_ratio,
            fovy: PI / 2.0,
            znear: 0.01,
            zfar: 300.0,
            speed: 0.05,
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

    pub fn build_mvp(&self) -> [[f32; 4]; 4] {
        let gaze = self.get_gaze();
        let view = Mat4::look_to_rh(self.position, gaze, self.up);
        let projection = Mat4::perspective_rh(self.fovy, self.aspect_ratio, self.znear, self.zfar);

        ((projection * view).to_cols_array_2d())
    }

    pub fn update_rotation(&mut self, delta: Vec2) {
        let sensitivity = 0.002;
        self.yaw -= delta.x * sensitivity;
        self.pitch -= delta.y * sensitivity;
        if self.pitch >= f32::to_radians(89.9) {
            self.pitch = f32::to_radians(89.9);
        } else if self.pitch <= f32::to_radians(-89.9) {
            self.pitch = f32::to_radians(-89.9);
        }
    }

    /// Now we use data directly from the input manager to update the camera position
    /// This is temporary until we define the player's state.
    pub fn update(&mut self, player_state: &InputManager) {
        let player_state = player_state.to_key_record();
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
