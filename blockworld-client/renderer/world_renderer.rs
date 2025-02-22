use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use wgpu::*;

use crate::game::client::BlockworldClient;

use super::{
    bytes_provider::StaticBytesProvider,
    camera::Camera,
    input_manager::InputManager,
    meshing::meshing_manager::{self, MeshingManager},
    pipeline::{RegularPipeline, WireframePipeline},
    resource_manager::BLOCK_ATLAS,
    shaders::WgslShader,
    texture::{BindableTexture, TextureWithView},
    uniform::{ToBytes, Uniform},
};

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct RawMat4(pub [[f32; 4]; 4]);
impl ToBytes for RawMat4 {}
impl From<Mat4> for RawMat4 {
    fn from(mat: Mat4) -> Self {
        Self(mat.to_cols_array_2d())
    }
}

pub struct WorldRenderer {
    pub camera: Camera,

    pub debug_mode: bool,
    pub depth_texture: TextureWithView,

    diffuse_texture: BindableTexture,
    game: BlockworldClient,

    main_pipeline: RegularPipeline,
    matrix_uniform: Uniform<RawMat4>,

    meshing_manager: MeshingManager,
    wireframe_pipeline: WireframePipeline,
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
        matrix_uniform.update(queue, camera.build_mvp().into());

        let diffuse_texture = BindableTexture::new(
            &device,
            &queue,
            &image::DynamicImage::ImageRgba8(BLOCK_ATLAS.get_image().clone()),
            Some("Diffuse Texture"),
        );

        let depth_texture = TextureWithView::new_depth(&device, &config);

        let shader = WgslShader::new(
            &"minecraft:assets/shaders/default_shader.wgsl".into(),
            &StaticBytesProvider,
            device,
            "fs",
            "vs",
        )
        .expect("Failed to load shader");

        let wireframe_shader = WgslShader::new(
            &"minecraft:assets/shaders/wireframe_shader.wgsl".into(),
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

        let game = BlockworldClient::new();
        let meshing_manager = MeshingManager::new();

        Self {
            debug_mode: false,
            main_pipeline,
            wireframe_pipeline,
            diffuse_texture,
            depth_texture,
            camera,
            matrix_uniform,
            game,
            meshing_manager,
        }
    }

    pub fn update(&mut self, queue: &Queue, input: &InputManager) {
        // Move the camera based on user input
        self.camera.update(input);

        // Update the uniform buffer with the new camera matrix
        self.matrix_uniform
            .update(queue, self.camera.build_mvp().into());
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

        {
            self.meshing_manager.render(rpass);
        }
    }
}
