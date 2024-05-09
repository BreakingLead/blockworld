use std::f32::consts::PI;

use glam::vec3;
use log::info;
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, KeyCode, PhysicalKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::Window,
};

use crate::{
    render::camera::{Camera, MatrixUniform},
    render::texture,
    render::vertex::{Vertex, INDICES, VERTICES},
};

pub struct State<'a> {
    pub window: Window,

    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    texture: texture::Texture,
    texture_bind_group: wgpu::BindGroup,

    camera: Camera,
    matrix_uniform: MatrixUniform,
    matrix_buffer: wgpu::Buffer,
    matrix_bind_group: wgpu::BindGroup,

    timer: u64,
}

impl<'a> State<'a> {
    pub async fn new(event_loop: &EventLoop<()>) -> State<'a> {
        unsafe {
            // /-------------------
            // Create the window
            let window = event_loop
                .create_window(Window::default_attributes().with_title("Blockworld Indev"))
                .unwrap();
            window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
            window.set_cursor_visible(false);

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
            let surface = instance
                .create_surface(&*(&window as *const Window))
                .unwrap();

            // Adapter is used to create device and queue.
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();

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
                .await
                .unwrap();

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
                include_bytes!("../assets/brick.png"),
                "assets/brick.png",
            )
            .unwrap();

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
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            // Create the vertex buffer.
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

            Self {
                window,

                surface,
                device,
                queue,
                config,
                size,
                render_pipeline,

                vertex_buffer,
                index_buffer,

                texture,
                texture_bind_group,

                camera,
                matrix_buffer,
                matrix_uniform,
                matrix_bind_group,

                timer: 0,
            }
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera
                .update_aspect_ratio(new_size.width as f32 / new_size.height as f32);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn device_input(&mut self, device_event: &DeviceEvent, event_loop: &ActiveEventLoop) {
        match device_event {
            DeviceEvent::MouseMotion { delta } => {
                let sensitivity = 0.002;
                self.camera.yaw -= delta.0 as f32 * sensitivity;
                self.camera.pitch -= delta.1 as f32 * sensitivity;
                if self.camera.pitch >= f32::to_radians(89.9) {
                    self.camera.pitch = f32::to_radians(89.0);
                } else if self.camera.pitch <= f32::to_radians(-89.9) {
                    self.camera.pitch = f32::to_radians(-89.9);
                }
            }
            _ => (),
        }
    }
    pub fn window_input(&mut self, window_event: &WindowEvent, event_loop: &ActiveEventLoop) {
        match window_event {
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                if state.is_pressed() {
                    match button {
                        winit::event::MouseButton::Left => {
                            info!("Left clicked!");
                        }
                        _ => (),
                    }
                }
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }

                match event.key_without_modifiers().as_ref() {
                    Key::Character("w") => {
                        self.camera.go_forward(1.0);
                    }
                    Key::Character("a") => {
                        self.camera.go_right(-1.0);
                    }
                    Key::Character("s") => {
                        self.camera.go_forward(-1.0);
                    }
                    Key::Character("d") => {
                        self.camera.go_right(1.0);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        // if self.timer % 50 == 0 {
        //     dbg!(self.camera.position);
        // }
        self.matrix_uniform.update_matrix(&self.camera);
        self.queue.write_buffer(
            &self.matrix_buffer,
            0,
            bytemuck::cast_slice(&[self.matrix_uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.matrix_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}