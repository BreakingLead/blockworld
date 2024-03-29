use std::f32::consts::{FRAC_PI_2, PI};

use glam::{mat4, vec3, vec4, Mat4, Vec3};
use winit_input_helper::WinitInputHelper;
type Radian = f32;


/// A camera data struct.
#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    yaw: Radian,
    pitch: Radian,
    // Perseved roll
    // roll: Rad<f32>,
}

impl Camera {
    pub fn new(position: Vec3, yaw: Radian, pitch: Radian) -> Self {
        Self {
            position,
            yaw: yaw,
            pitch: pitch,
        }
    }

    /// Returns the direction of the camera.
    pub fn get_gaze(&self) -> Vec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        vec3(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn get_model_view_matrix(&self) -> Mat4 {
        Mat4::look_to_rh(self.position, self.get_gaze(), Vec3::Y)
    }
}

/// The struct to create projection matrix.
#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    fovy: f32, // Radian
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fovy: Radian, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

#[derive(Debug)]
pub struct CameraController {
    is_left: bool,
    is_right: bool,
    is_forward: bool,
    is_backward: bool,
    is_up: bool,
    is_down: bool,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            is_left: false,
            is_right: false,
            is_forward: false,
            is_backward: false,
            is_up: false,
            is_down: false,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    // FIXME: If we press 2 keys simultaneously there will be a bug.
    pub fn process_input(&mut self, input: &WinitInputHelper) {
        use winit::keyboard::KeyCode;
        if input.key_held(KeyCode::KeyW) {
            self.is_forward = true;
        } else if input.key_held(KeyCode::KeyS) {
            self.is_backward = true;
        } else if input.key_held(KeyCode::KeyA) {
            self.is_left = true;
        } else if input.key_held(KeyCode::KeyD) {
            self.is_right = true;
        } else if input.key_held(KeyCode::Space) {
            self.is_up = true;
        } else if input.key_held(KeyCode::ShiftLeft) {
            self.is_down = true;
        } else {
            self.is_backward = false;
            self.is_forward = false;
            self.is_left = false;
            self.is_right = false;
            self.is_up = false;
            self.is_down = false;
        }
        (self.rotate_horizontal, self.rotate_vertical) = input.mouse_diff();
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: std::time::Duration) {
        log::info!("{:#?}", camera);
        let mut forward = camera.get_gaze().normalize();
        forward.y = 0.0;
        let right = camera.get_gaze().cross(Vec3::Y).normalize();

        let direction = {
            let fb_val = {
                let f_val = f32::from(self.is_forward);
                let b_val = -f32::from(self.is_backward);
                ((f_val + b_val) * forward).normalize_or_zero()
            };
            let rl_val = {
                let r_val = f32::from(self.is_right);
                let l_val = -f32::from(self.is_left);
                ((l_val + r_val) * right).normalize_or_zero()
            };
            let ud_val = {
                let u_val = f32::from(self.is_up);
                let d_val = -f32::from(self.is_down);
                ((u_val + d_val) * Vec3::Y).normalize_or_zero()
            };

            fb_val + rl_val + ud_val
        };

        camera.position = camera.position + dt.as_secs_f32() * direction * self.speed;
        camera.yaw = camera.yaw + self.rotate_horizontal * self.sensitivity * dt.as_secs_f32();
        camera.pitch = camera.pitch - self.rotate_vertical * self.sensitivity * dt.as_secs_f32();

        // Keep the camera's angle from going too high/low.
        const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.001;
        if camera.pitch < -(SAFE_FRAC_PI_2) {
            camera.pitch = -(SAFE_FRAC_PI_2);
        } else if camera.pitch > (SAFE_FRAC_PI_2) {
            camera.pitch = SAFE_FRAC_PI_2;
        }

        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            // view_position: [0.0;4],
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update(&mut self, camera: &Camera, projection: &Projection) {
        // self.view_position = camera.position.extend(1.0).into();
        self.view_proj = (projection.get_projection_matrix() * camera.get_model_view_matrix())
            .to_cols_array_2d();
    }
}