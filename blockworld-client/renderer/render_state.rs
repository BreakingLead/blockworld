//! Central render state: owns the wgpu device, surface, and `WorldRenderer`.
//!
//! This is the glue between `window_init` (event loop) and `world_renderer`
//! (actual rendering). It manages the swapchain lifecycle and delegates
//! per-frame work.

use std::sync::Arc;
use std::time::Instant;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::debug_ui::DebugUi;
use super::init_helpers;
use super::input_manager::InputManager;
use super::world_renderer::WorldRenderer;

pub struct RenderState {
    pub window: Arc<Window>,
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,

    pub input_manager: InputManager,
    pub world_renderer: WorldRenderer,
    pub debug_ui: DebugUi,

    dt_timer: Instant,
    fps: f32,
}

impl RenderState {
    pub fn new(window: Window) -> Option<Self> {
        let window = Arc::new(window);
        let size = window.inner_size();

        let instance = init_helpers::create_instance();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = init_helpers::create_adapter(&instance, &surface)?;
        let (device, queue) = init_helpers::create_device_and_queue(&adapter);
        let config = init_helpers::create_surface_config(size, &surface, &adapter);
        surface.configure(&device, &config);

        let world_renderer = WorldRenderer::new(&device, &config, &queue, size);
        let debug_ui = DebugUi::new(&device, config.format);

        Some(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            input_manager: InputManager::default(),
            world_renderer,
            debug_ui,
            dt_timer: Instant::now(),
            fps: 0.0,
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.world_renderer.resize(
            &self.device,
            &self.config,
            new_size.width as f32 / new_size.height as f32,
        );
        self.surface.configure(&self.device, &self.config);
        self.size = new_size;
    }

    pub fn update(&mut self) {
        let dt = self.dt_timer.elapsed();
        self.dt_timer = Instant::now();
        self.fps = 1.0 / dt.as_secs_f32();

        self.world_renderer
            .update(&self.queue, &self.device, &self.input_manager);

        // Build egui UI (stored for later render pass)
        self.debug_ui.run(
            &self.window,
            self.fps,
            self.world_renderer.game.chunks.chunks.len(),
            self.world_renderer.game.pending_generation_count(),
            self.world_renderer.meshing_manager.render_map_len(),
            self.world_renderer.camera.position,
        );
    }

    pub fn render(&mut self) -> Result<(), ()> {
        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(tex) => tex,
            CurrentSurfaceTexture::Suboptimal(tex) => {
                self.surface.configure(&self.device, &self.config);
                tex
            }
            CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return Err(());
            }
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Err(()),
            CurrentSurfaceTexture::Validation => {
                log::error!("Surface validation error");
                return Err(());
            }
        };

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Encoder"),
            });

        // Pass 1: 3D world
        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("World"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.51,
                            g: 0.66,
                            b: 0.98,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.world_renderer.depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            self.world_renderer.render(&mut rpass);
        }

        // Pass 2: egui debug overlay (Load, not Clear — preserves 3D scene)
        self.debug_ui
            .render(&self.device, &self.queue, &mut encoder, &view);

        self.queue.submit([encoder.finish()]);
        output.present();
        Ok(())
    }
}
