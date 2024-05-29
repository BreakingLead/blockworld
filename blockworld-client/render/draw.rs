use std::{f32::consts::PI, fmt::format};

use anyhow::*;
use glam::{vec2, vec3};
use log::{debug, info};
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::{DeviceEvent, KeyEvent, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{Key, KeyCode, NamedKey, PhysicalKey}, platform::modifier_supplement::KeyEventExtModifierSupplement, window::{self, Fullscreen, Window, WindowAttributes}
};

use crate::{
    game::{
        block::{BlockMeta, BlockType, ResourceLocation},
        chunk::Chunk,
        player_state::PlayerState,
        register::RegisterTable,
        Game,
    },
    io::{atlas_helper::AtlasMeta, input_helper::InputState},
    render::{
        camera::{Camera, MatrixUniform},
        texture,
        vertex::Vertex,
    }, BootArgs,
};

use super::render_chunk::RenderChunk;
use crate::io::atlas_helper::AtlasCoordinate;

pub struct State<'a> {
    pub window: Window,

    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,

    pub render_chunk: RenderChunk,

    pub texture: texture::Texture,
    pub texture_bind_group: wgpu::BindGroup,

    pub depth_texture: texture::Texture,

    pub camera: Camera,
    pub matrix_uniform: MatrixUniform,
    pub matrix_buffer: wgpu::Buffer,
    pub matrix_bind_group: wgpu::BindGroup,

    // The Game
    pub game: Game,
    pub input_state: crate::io::input_helper::InputState,

    pub register_table: RegisterTable,

    pub timer: u64,
}

impl<'a> State<'a> {
    pub async fn new(event_loop: &EventLoop<()>, boot_args:&BootArgs) -> Result<State<'a>> {
        // /-------------------../assets/atlas.png
        // Create the window
        let mut window_attrs = Window::default_attributes()
                .with_title("Blockworld Indev");
        // set screen size based on boot_args 
        if boot_args.full_screen{
            window_attrs = window_attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }else{
            window_attrs = window_attrs.with_inner_size(PhysicalSize::new(boot_args.width,boot_args.height))
        }
        let window = event_loop
            .create_window(window_attrs)?;
        window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
        window.set_cursor_visible(false);

        let player_state: PlayerState = Default::default();

        let size = window.inner_size();
        // \-------------------

        // /-------------------
        // Instance is the way to create surface and adapter.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        // \-------------------

        // /-------------------
        // Generate & Configure the surface
        let surface = unsafe {
            instance
                .create_surface(&*(&window as *const Window))?
        };

        // Adapter is used to create device and queue.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await.with_context(|| format!("adapter created error for problem with wgpu"))?;            

        // Device is the abstraction of the GPU. Queue is the command queue to send to GPU.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
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
        // \-------------------

        // /-------------------
        // Camera thingy
        let camera = Camera::new(size.width as f32 / size.height as f32);

        let mut matrix_uniform = MatrixUniform::new();
        matrix_uniform.update_matrix(&camera);

        let matrix_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Martix Buffer"),
            contents: bytemuck::cast_slice(&[matrix_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let matrix_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 30,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let matrix_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &matrix_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 30,
                resource: matrix_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        // \-------------------

        // /-------------------
        // Texture & its bind group
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

        let depth_texture = texture::Texture::create_depth_texture(&device, &config);
        // \-------------------

        let shader = device.create_shader_module(include_wgsl!("shaders/default_shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &matrix_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // -------------------
        // | Game Initialize |
        // -------------------

        let (image_w, image_h) = image::io::Reader::open("../assets/atlas.png")?
            .into_dimensions()?;
            
        let atlas_meta = AtlasMeta {
            tile_w: 16,
            tile_h: 16,
            image_w,
            image_h,
        };
        let mut register_table = RegisterTable::new();
        register_table.register_block(
            1,
            BlockMeta {
                name: ResourceLocation::new("test_a"),
                ty: BlockType::Solid,
                atlas_coord: [atlas_meta.get(6, 19)?; 6],
            },
        );
        register_table.register_block(
            2,
            BlockMeta {
                name: ResourceLocation::new("test_b"),
                ty: BlockType::Solid,
                atlas_coord: [atlas_meta.get(16, 6)?; 6],
            },
        );

        let chunk = Chunk::default();
        let render_chunk = RenderChunk::new(&device, &chunk, &register_table);

        let game = Game::default();
        let input_state = InputState::default();

        Ok(Self {
            window,

            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,

            texture,
            texture_bind_group,

            depth_texture,

            render_chunk,

            camera,
            matrix_buffer,
            matrix_uniform,
            matrix_bind_group,

            timer: 0,

            input_state,
            game,

            register_table,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera
                .update_aspect_ratio(new_size.width as f32 / new_size.height as f32);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        self.game.update(&self.input_state);
        self.camera.update(&self.game.player_state);
        self.matrix_uniform.update_matrix(&self.camera);
        self.queue.write_buffer(
            &self.matrix_buffer,
            0,
            bytemuck::cast_slice(&[self.matrix_uniform]),
        );
    }

    pub fn render(&mut self) -> std::result::Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.matrix_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.render_chunk.vertex_buffer.slice(..));
            render_pass.draw(0..self.render_chunk.vertex_count, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        std::result::Result::Ok(())
    }
}
