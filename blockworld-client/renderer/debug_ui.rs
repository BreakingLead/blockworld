//! egui-based debug overlay.

use egui::Context;
use egui_wgpu::Renderer;
use wgpu::{Device, Queue, TextureFormat};

pub struct DebugUi {
    renderer: Renderer,
    context: Context,
    paint_jobs: Vec<egui::epaint::ClippedPrimitive>,
    screen: Option<egui_wgpu::ScreenDescriptor>,
}

impl DebugUi {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        Self {
            renderer: Renderer::new(device, format, Default::default()),
            context: Context::default(),
            paint_jobs: Vec::new(),
            screen: None,
        }
    }

    pub fn run(
        &mut self,
        window: &winit::window::Window,
        fps: f32,
        chunk_count: usize,
        chunk_pending: usize,
        mesh_count: usize,
        cam_pos: glam::Vec3,
    ) {
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(),
                egui::vec2(
                    window.inner_size().width as f32,
                    window.inner_size().height as f32,
                ),
            )),
            ..Default::default()
        };

        let output = self.context.run(raw, |ctx| {
            egui::Window::new("Debug").show(ctx, |ui| {
                ui.label(format!("FPS: {:.0}", fps));
                ui.label(format!(
                    "Camera: ({:.1}, {:.1}, {:.1})",
                    cam_pos.x, cam_pos.y, cam_pos.z
                ));
                ui.label(format!("Chunks: {} ({} pending)", chunk_count, chunk_pending));
                ui.label(format!("Meshes: {}", mesh_count));
                ui.separator();
                ui.label("WASD = move  Space/Shift = fly");
                ui.label("F1 = wireframe  Esc = quit");
            });
        });

        self.paint_jobs = self
            .context
            .tessellate(output.shapes, output.pixels_per_point);
        self.screen = Some(egui_wgpu::ScreenDescriptor {
            size_in_pixels: [window.inner_size().width, window.inner_size().height],
            pixels_per_point: window.scale_factor() as f32,
        });
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) {
        let screen = match &self.screen {
            Some(s) => s,
            None => return,
        };

        if self.paint_jobs.is_empty() {
            return;
        }

        self.renderer
            .update_buffers(device, queue, encoder, &self.paint_jobs, screen);

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });
        let rpass_static: &mut wgpu::RenderPass<'static> =
            unsafe { std::mem::transmute(&mut rpass) };
        self.renderer.render(rpass_static, &self.paint_jobs, screen);
        drop(rpass);
    }
}
