use crate::renderer::init_helpers::*;
use crate::renderer::world_renderer::{self, WorldRenderer};
use egui_winit_platform::Platform;
use std::{sync::Arc, time::Instant};
use wgpu::{include_wgsl, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use super::input_manager::InputManager;
use super::uniform::{RawMat4, Uniform};

/// The RenderState struct holds all the state needed to render the game's user interface and game world.
pub struct RenderState {
    pub window: Arc<Window>,
    pub surface: Surface<'static>,

    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,

    pub input_manager: InputManager,

    pub dt_timer: Instant,
    pub global_timer: Instant,

    pub world_renderer: WorldRenderer,
}

impl RenderState {
    pub fn new(window: Window) -> RenderState {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        let instance = create_instance();
        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let adapter = create_adapter(&instance, &surface);
        let (device, queue) = create_device_and_queue(&adapter);
        let config = create_surface_config(size, &surface, &adapter);
        surface.configure(&device, &config);
        let input_manager = InputManager::default();

        let world_renderer = WorldRenderer::new(&device, &config, &queue, size);

        Self {
            window: window_arc,
            surface,
            device,
            queue,
            config,
            size,
            input_manager,
            world_renderer,
            dt_timer: Instant::now(),
            global_timer: Instant::now(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.world_renderer.resize(
                &self.queue,
                &self.device,
                &self.config,
                new_size.width as f32 / new_size.height as f32,
            );
            self.surface.configure(&self.device, &self.config);
            self.size = new_size;
        }
    }

    pub fn update(&mut self) {
        // Time between this and the previous frame
        let delta_time = self.dt_timer.elapsed();
        // Set the timer to 0
        self.dt_timer = Instant::now();

        self.window.set_title(
            format!(
                "Blockworld Dev [fps: {:.0}]",
                1.0 / delta_time.as_secs_f32()
            )
            .as_str(),
        );

        self.world_renderer.update(&self.queue, &self.input_manager);
    }

    pub fn render(&mut self) {
        let output_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to get window surface texture");

        let output_texture_view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Blockworld Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Blockworld Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        // #82a9f9 minecraft's sky blue
                        r: 0.509804,
                        g: 0.662745,
                        b: 0.976471,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.world_renderer.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        self.world_renderer.render(&mut render_pass);
        drop(render_pass);

        let command_buffer = encoder.finish();
        self.queue.submit([command_buffer]);
        output_texture.present();
    }
}
