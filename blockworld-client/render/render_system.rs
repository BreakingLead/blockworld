use anyhow::{Context, Result};
use wgpu::include_wgsl;
use wgpu_text::{
    glyph_brush::{ab_glyph::FontRef, OwnedSection},
    TextBrush,
};
use winit::window::Window;

use super::{
    camera::{Camera, MatrixData},
    pipeline::{RegularPipeline, WireframePipeline},
    render_array::RenderArray,
    texture::Texture,
    uniform::Uniform,
};

pub struct RenderSystem<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub main_pipeline: RegularPipeline,
    pub wireframe_pipeline: WireframePipeline,

    pub render_array: RenderArray,

    pub texture: Texture,
    pub texture_bind_group: wgpu::BindGroup,

    pub depth_texture: Texture,
    // UI
    // pub fps_text_section: OwnedSection,
    // pub brush: TextBrush<FontRef<'a>>,
    pub camera: Camera,
    pub matrix_uniform: Uniform<MatrixData>,
}

impl RenderSystem<'_> {
    pub async fn new(window: Window) -> Result<Self> {
        // Instance is the way to create surface and adapter.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let size = window.inner_size();

        let camera = Camera::new();

        // Generate & Configure the surface
        let surface = unsafe { instance.create_surface(&*(&window as *const Window))? };

        // Adapter is used to create device and queue.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .with_context(|| format!("adapter created error for problem with wgpu"))?;

        // Device is the abstraction of the GPU. Queue is the command queue to send to GPU.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::POLYGON_MODE_LINE,
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let texture = crate::render::texture::Texture::from_bytes(
            &device,
            &queue,
            include_bytes!("../assets/atlas.png"),
            "Block Texture",
        )?;

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 20,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 21,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 20,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 21,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let depth_texture = Texture::create_depth_texture(&device, &config);
        // \-------------------

        let shader = device.create_shader_module(include_wgsl!("shaders/default_shader.wgsl"));
        let wireframe_shader =
            device.create_shader_module(include_wgsl!("shaders/debug_shader.wgsl"));

        let mut matrix_uniform = Uniform::new(
            &device,
            Box::new(MatrixData::new()),
            30,
            Some("Matrix Uniform"),
        );

        let main_pipeline = RegularPipeline::new(
            &device,
            &[&texture_bind_group_layout, &matrix_uniform.layout],
            &shader,
            &config,
        );

        let wireframe_pipeline = WireframePipeline::new(
            &device,
            &[&texture_bind_group_layout, &matrix_uniform.layout],
            &wireframe_shader,
            &config,
        );

        // let brush = wgpu_text::BrushBuilder::using_font_bytes(settings.font)
        //     .unwrap()
        //     .with_depth_stencil(Some(wgpu::DepthStencilState {
        //         format: wgpu::TextureFormat::Depth32Float,
        //         depth_write_enabled: false,
        //         depth_compare: wgpu::CompareFunction::LessEqual,
        //         stencil: wgpu::StencilState::default(),
        //         bias: wgpu::DepthBiasState::default(),
        //     }))
        //     .build(&device, config.width, config.height, config.format);

        // let fps_text_section = Section::default()
        //     .add_text(
        //         Text::new("Hello World Test AAAAAAAAAAAAA")
        //             .with_color([1.0, 1.0, 1.0, 1.0])
        //             .with_scale(25.0),
        //     )
        //     .with_layout(Layout::default().v_align(wgpu_text::glyph_brush::VerticalAlign::Center))
        //     .with_screen_position((50.0, config.height as f32 * 0.5))
        //     .to_owned();

        let render_array = RenderArray::new(&mut game.chunk_provider, &device, &register_table);

        Ok(RenderSystem {
            surface,
            device,
            queue,
            surface_config: config,
            size,
            main_pipeline,
            wireframe_pipeline,
            render_array,
            texture,
            texture_bind_group,
            depth_texture,
            // fps_text_section,
            // brush,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera
                .update_aspect_ratio(new_size.width as f32 / new_size.height as f32);

            self.brush
                .resize_view(new_size.width as f32, new_size.height as f32, &self.queue);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);
            self.depth_texture = Texture::create_depth_texture(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {
        self.camera.update(&self.game.player_state);

        self.matrix_uniform.uniform.update_matrix(&self.camera);
        self.queue.write_buffer(
            &self.matrix_uniform.buffer,
            0,
            bytemuck::cast_slice(&[*self.matrix_uniform.uniform]),
        );
    }
}
