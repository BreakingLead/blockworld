use glam::Mat4;
use wgpu::*;

use crate::{
    game::{input_manager::InputManager, key_record::MovementRecord},
    world::chunk_array::ChunkArray,
};

use super::{
    bytes_provider::StaticBytesProvider,
    camera::Camera,
    chunk::render_chunk::RenderChunk,
    resource_manager::BLOCK_ATLAS,
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

    pub camera: Camera,
    matrix_uniform: Uniform<RawMat4>,

    chunks: Box<ChunkArray>,
    render_array: Vec<RenderChunk>,
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
        matrix_uniform.update(queue, camera.build_mvp());

        let diffuse_texture = BindableTexture::new(
            &device,
            &queue,
            &image::DynamicImage::ImageRgba8(BLOCK_ATLAS.get_image().clone()),
            Some("Diffuse Texture"),
        );

        let depth_texture = TextureWithView::new_depth(&device, &config);

        let shader = WgslShader::new(
            &"blockworld:assets/shaders/default_shader.wgsl".into(),
            &StaticBytesProvider,
            device,
            "fs",
            "vs",
        )
        .expect("Failed to load shader");

        let wireframe_shader = WgslShader::new(
            &"blockworld:assets/shaders/wireframe_shader.wgsl".into(),
            &StaticBytesProvider,
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

        let chunks = Box::new(ChunkArray::new(8));
        let mut render_array = vec![];
        for (loc, chunk) in chunks.chunks.iter() {
            render_array.push(RenderChunk::new(device, chunk));
        }

        Self {
            debug_mode: false,
            main_pipeline,
            wireframe_pipeline,
            diffuse_texture,
            depth_texture,
            camera,
            matrix_uniform,
            render_array,
            chunks,
        }
    }

    pub fn update(&mut self, queue: &Queue) {
        // Move the camera based on user input
        self.camera.update(todo!());

        // Update the uniform buffer with the new camera matrix
        self.matrix_uniform.update(queue, self.camera.build_mvp());
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
        if self.debug_mode {
            // render with wireframe
            rpass.set_pipeline(&self.wireframe_pipeline.pipeline);
        } else {
            // render with texture
            rpass.set_pipeline(&self.main_pipeline.pipeline);
        }

        rpass.set_bind_group(0, &self.diffuse_texture.bind_group, &[]);
        rpass.set_bind_group(1, &self.matrix_uniform.bind_group, &[]);

        for chunk in self.render_array.iter() {
            rpass.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
            rpass.draw(0..chunk.vertex_count, 0..1);
        }
    }
}
