use crate::{
    game::{settings::Settings, Game},
    get_cli_args,
    renderer::{
        camera::{Camera, MatrixData},
        pipeline::{RegularPipeline, WireframePipeline},
        uniform::{RawMat4, Uniform},
    },
};
use glam::Mat4;
use std::{sync::Arc, time::Instant};
use wgpu::*;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Fullscreen, Window},
};
/// state contains all things the game needs
pub struct RenderState {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,

    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,

    pub main_pipeline: RegularPipeline,
    pub wireframe_pipeline: WireframePipeline,

    pub texture: Texture,
    pub texture_bind_group: wgpu::BindGroup,

    pub depth_texture: Texture,

    pub camera: Camera,
    pub matrix_uniform: Uniform<MatrixData>,

    // IO
    pub input_state: InputState,

    // The Game
    pub game: Game,
    pub fps: f32,
    pub dt_timer: Instant,
    pub global_timer: Instant,

    // UI
    // pub fps_text_section: OwnedSection,
    // pub brush: TextBrush<FontRef<'a>>,

    // Settings
    pub settings: Settings<'static>,

    pub register_table: RegisterTable,

    // Debug
    pub debug_mode: bool,
}

fn create_window(event_loop: &EventLoop<()>) -> Window {
    let mut window_attrs = Window::default_attributes().with_title("Blockworld Indev");
    let args = get_cli_args();
    // set screen size based on boot_args
    if args.full_screen {
        window_attrs = window_attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
    } else {
        window_attrs = window_attrs.with_inner_size(PhysicalSize::new(args.width, args.height))
    }
    let window = event_loop.create_window(window_attrs).unwrap();
    window.set_cursor_grab(winit::window::CursorGrabMode::Confined)?;
    window.set_cursor_visible(false);

    window
}

fn create_instance() -> Instance {
    Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    })
}

fn create_surface<'window>(
    instance: &Instance,
    window: &'window Window,
) -> Result<Surface<'window>, wgpu::CreateSurfaceError> {
    instance.create_surface(window)
}

async fn create_adapter(instance: &Instance, surface: &Surface<'_>) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap()
}

async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::POLYGON_MODE_LINE,
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();
    (device, queue)
}

fn create_surface_config(
    size: PhysicalSize<u32>,
    surface: &Surface,
    adapter: &Adapter,
) -> wgpu::SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::AutoNoVsync,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

impl RenderState {
    pub async fn new(window: Window) -> RenderState {
        let window_arc = Arc::new(window);
        let size = window.inner_size();
        let instance = create_instance();
        let surface = create_surface(&instance, &window).unwrap();
        let adapter = create_adapter(&instance, &surface).await;
        let (device, queue) = create_device_and_queue(&adapter).await;
        let config = create_surface_config(size, &surface, &adapter);
        surface.configure(&device, &config);

        // Camera thingy
        let camera = Camera::new(size.width as f32 / size.height as f32);

        let mut matrix_uniform = Uniform::new(
            &device,
            RawMat4::from(Mat4::IDENTITY),
            30,
            Some("Matrix Uniform"),
        );
        matrix_uniform.uniform.as_mut() = camera.build_mvp();

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

        let shader = device.create_shader_module(include_wgsl!("../shaders/default_shader.wgsl"));
        let wireframe_shader =
            device.create_shader_module(include_wgsl!("../shaders/debug_shader.wgsl"));

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

        // -------------------
        // | Game Initialize |
        // -------------------

        let mut register_table = RegisterTable::new();
        let atlas = Atlas::new("assets/atlas.png", 16);

        let mut game = Game::default();
        let render_array = RenderArray::new(&mut game.chunk_provider, &device, &register_table);
        let input_state = InputState::default();

        let brush = wgpu_text::BrushBuilder::using_font_bytes(settings.font)
            .unwrap()
            .with_depth_stencil(Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }))
            .build(&device, config.width, config.height, config.format);

        Ok(Self {
            window,

            surface,
            device,
            queue,
            config,
            size,
            main_pipeline,
            wireframe_pipeline,

            texture,
            texture_bind_group,

            depth_texture,

            camera,
            matrix_uniform,

            settings,
            fps: 0.0,
            dt_timer: Instant::now(),
            global_timer: Instant::now(),

            input_state,
            game,

            register_table,
            debug_mode: false,
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
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
        // Time between this and the previous frame
        let delta_time = self.dt_timer.elapsed();
        // Set the timer to 0
        self.dt_timer = Instant::now();

        // Game Update
        self.game.update(&self.input_state);

        self.window.set_title(
            format!(
                "Blockworld Dev [fps: {:.0}]",
                1.0 / delta_time.as_secs_f32()
            )
            .as_str(),
        );

        // Camera Update
        self.camera.update(&self.game.player_state);

        self.matrix_uniform.uniform.update_matrix(&self.camera);
        self.queue.write_buffer(
            &self.matrix_uniform.buffer,
            0,
            bytemuck::cast_slice(&[*self.matrix_uniform.uniform]),
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
                label: Some("Blockworld Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Blockworld Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 1.0,
                            b: 239.0 / 255.0,
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

            // check debug mode
            if self.debug_mode {
                // render with wireframe
                render_pass.set_pipeline(&self.wireframe_pipeline.pipeline);
            } else {
                // render with texture
                render_pass.set_pipeline(&self.main_pipeline.pipeline);
            }

            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.matrix_uniform.bind_group, &[]);

            for chunk in self.render_array.chunks().iter() {
                render_pass.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
                render_pass.draw(0..chunk.vertex_count, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        std::result::Result::Ok(())
    }

    /// read a line from cmd synchronously. It should't be run on main displaying thread
    pub async fn try_exec_single_instr_from_console(&mut self) -> Result<()> {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let mut console_string = String::new();
        handle.read_line(&mut console_string)?;
        // exec_instr_from_string(console_string, self).await?;
        Ok(())
    }
}
