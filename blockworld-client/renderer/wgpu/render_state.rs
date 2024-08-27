use crate::block::Block;
use crate::game::input_manager::InputManager;
use crate::renderer::wgpu::init_helpers::*;
use crate::renderer::world_renderer::{self, WorldRenderer};
use crate::renderer::{camera::*, wgpu::pipeline::*};
use blockworld_utils::Registry;
use glam::Mat4;
use std::{sync::Arc, time::Instant};
use wgpu::{include_wgsl, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use super::texture::TextureWithView;
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

        {
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
        }

        let command_buffer = encoder.finish();
        self.queue.submit([command_buffer]);
        output_texture.present();
    }

    // read a line from cmd synchronously. It should't be run on main displaying thread
    // pub async fn try_exec_single_instr_from_console(&mut self) -> anyhow::Result<()> {
    //     let stdin = std::io::stdin();
    //     let mut handle = stdin.lock();
    //     let mut console_string = String::new();
    //     handle.read_line(&mut console_string)?;
    //     // exec_instr_from_string(console_string, self).await?;
    //     Ok(())
    // }
}
