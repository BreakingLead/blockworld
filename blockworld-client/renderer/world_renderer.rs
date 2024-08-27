use glam::Mat4;
use wgpu::*;

use crate::game::{input_manager::InputManager, key_record::MovementRecord};

use super::{
    camera::Camera,
    resource_provider::StaticResourceProvider,
    shaders::WgslShader,
    wgpu::{
        pipeline::{RegularPipeline, WireframePipeline},
        texture::{BindableTexture, TextureWithView},
        uniform::{RawMat4, Uniform},
    },
};

pub struct WorldRenderer {
    pub debug_mode: bool,

    main_pipeline: RegularPipeline,
    wireframe_pipeline: WireframePipeline,

    diffuse_texture: BindableTexture,
    pub depth_texture: TextureWithView,

    camera: Camera,
    matrix_uniform: Uniform<RawMat4>,
}

impl WorldRenderer {
    pub fn new(
        device: &Device,
        config: &wgpu::SurfaceConfiguration,
        queue: &Queue,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        // Camera thingy
        let camera = Camera::new(size.width as f32 / size.height as f32);

        let mut matrix_uniform = Uniform::new(
            &device,
            RawMat4::from(Mat4::IDENTITY),
            30,
            Some("Matrix Uniform"),
        );
        matrix_uniform.update(camera.build_mvp());

        let diffuse_texture =
            BindableTexture::new(&device, &queue, todo!(), Some("Diffuse Texture"));

        let depth_texture = TextureWithView::new_depth(&device, &config);

        let shader = WgslShader::new(
            &"blockworld:assets/shaders/default_shader.wgsl".into(),
            &StaticResourceProvider,
            device,
            "fs",
            "vs",
        )
        .expect("Failed to load shader");

        let wireframe_shader = WgslShader::new(
            &"blockworld:assets/shaders/wireframe_shader.wgsl".into(),
            &StaticResourceProvider,
            device,
            "fs",
            "vs",
        )
        .expect("Failed to load shader");

        let main_pipeline = RegularPipeline::new(
            &device,
            &[&diffuse_texture.bind_group_layout, &matrix_uniform.layout],
            &shader,
            &config,
        );

        let wireframe_pipeline = WireframePipeline::new(
            &device,
            &[&diffuse_texture.bind_group_layout, &matrix_uniform.layout],
            &wireframe_shader,
            &config,
        );
    }

    pub fn update(&mut self, queue: &Queue, input: &InputManager) {
        // Move the camera based on user input
        self.camera.update(MovementRecord::mk(input));
        self.camera.update_rotation(input.get_mouse_delta());

        // Update the uniform buffer with the new camera matrix
        self.matrix_uniform.update(self.camera.build_mvp());

        // Upload the new uniform buffer to the GPU
        queue.write_buffer(&self.matrix_uniform.buffer, 0, &self.matrix_uniform);
    }

    pub fn resize(
        &mut self,
        _queue: &Queue,
        device: &Device,
        config: &SurfaceConfiguration,
        new_aspect_ratio: f32,
    ) {
        self.depth_texture = TextureWithView::new_depth(device, config);
        self.camera.update_aspect_ratio(new_aspect_ratio);
    }

    pub fn render<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {
        // check debug mode
        if self.debug_mode {
            // render with wireframe
            rpass.set_pipeline(&self.wireframe_pipeline.pipeline);
        } else {
            // render with texture
            rpass.set_pipeline(&self.main_pipeline.pipeline);
        }

        rpass.set_bind_group(0, &self.diffuse_texture.bind_group, &[]);
        rpass.set_bind_group(1, &self.matrix_uniform.bind_group, &[]);

        // for chunk in self.render_array.chunks().iter() {
        //     render_pass.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
        //     render_pass.draw(0..chunk.vertex_count, 0..1);
        // }
    }
}
