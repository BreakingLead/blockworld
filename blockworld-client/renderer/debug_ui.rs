//! egui-based debug overlay.

use egui::Context;
use egui_wgpu::Renderer;
use wgpu::{Device, Queue, TextureFormat};
use winit::window::Window;

pub struct DebugUi {
    renderer: Renderer,
    context: Context,
}

impl DebugUi {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        Self {
            renderer: Renderer::new(device, format, Default::default()),
            context: Context::default(),
        }
    }

    pub fn begin_frame(
        &mut self,
        window: &Window,
        fps: f32,
        chunk_count: usize,
        chunk_pending: usize,
        mesh_count: usize,
        cam_pos: glam::Vec3,
    ) {
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(),
                egui::vec2(
                    window.inner_size().width as f32,
                    window.inner_size().height as f32,
                ),
            )),
            ..Default::default()
        };
        self.context.begin_pass(raw_input);

        egui::Window::new("Debug").show(&self.context, |ui| {
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
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut wgpu::CommandEncoder,
        window: &Window,
        view: &wgpu::TextureView,
    ) {
        let output = self.context.end_pass();
        let paint_jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [window.inner_size().width, window.inner_size().height],
            pixels_per_point: window.scale_factor() as f32,
        };
        self.renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &screen);

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
        // SAFETY: rpass is dropped immediately after this call, before
        // the encoder is used again. The 'static bound is an egui_wgpu
        // requirement but we never keep the rpass alive.
        let rpass_static: &mut wgpu::RenderPass<'static> =
            unsafe { std::mem::transmute(&mut rpass) };
        self.renderer.render(rpass_static, &paint_jobs, &screen);
        drop(rpass);
    }
}
