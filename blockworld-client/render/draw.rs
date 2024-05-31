use std::fmt::Debug;
use std::thread::Thread;
use std::{io::BufRead, time::Instant};

use anyhow::*;
use glam::*;
use wgpu::{include_wgsl, util::DeviceExt};
use wgpu_text::{
    glyph_brush::{ab_glyph::FontRef, Layout, OwnedSection, OwnedText, Section, Text},
    TextBrush,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Fullscreen, Window},
};

use crate::{
    game::{
        block::{BlockMeta, BlockType, ResourceLocation},
        chunk::Chunk,
        console_instr::exec_instr_from_string,
        player_state::PlayerState,
        register::RegisterTable,
        settings::Settings,
        Game,
    },
    io::{atlas_helper::AtlasMeta, input_helper::InputState},
    BootArgs,
};

use super::camera::{Camera, MatrixData};
use super::texture::Texture;
use super::uniform::*;
use super::{
    pipeline::{RegularPipeline, WireframePipeline},
    render_chunk::RenderChunk,
};

/// state contains all things the game needs
pub struct State<'a> {
    pub window: Window,

    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub main_pipeline: RegularPipeline,
    pub wireframe_pipeline: WireframePipeline,

    pub render_chunk: RenderChunk,

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
    pub fps_text_section: OwnedSection,
    pub brush: TextBrush<FontRef<'a>>,

    // Settings
    pub settings: Settings<'a>,

    pub register_table: RegisterTable,

    // Debug
    pub debug_mode: bool,
}

impl<'a> State<'a> {
    pub async fn new(event_loop: &EventLoop<()>, boot_args: &BootArgs) -> Result<State<'a>> {
        // /-------------------../assets/atlas.png
        // Create the window
        let mut window_attrs = Window::default_attributes().with_title("Blockworld Indev");
        // set screen size based on boot_args
        if boot_args.full_screen {
            window_attrs = window_attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
        } else {
            window_attrs =
                window_attrs.with_inner_size(PhysicalSize::new(boot_args.width, boot_args.height))
        }
        let window = event_loop.create_window(window_attrs)?;
        window.set_cursor_grab(winit::window::CursorGrabMode::Confined)?;
        window.set_cursor_visible(false);

        let _player_state: PlayerState = Default::default();

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
        // \-------------------

        // /-------------------
        // Camera thingy
        let camera = Camera::new(size.width as f32 / size.height as f32);

        let mut matrix_uniform = Uniform::new(
            &device,
            Box::new(MatrixData::new()),
            30,
            Some("Matrix Uniform"),
        );
        matrix_uniform.uniform.as_mut().update_matrix(&camera);

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

        let depth_texture = Texture::create_depth_texture(&device, &config);
        // \-------------------

        let shader = device.create_shader_module(include_wgsl!("shaders/default_shader.wgsl"));
        let wireframe_shader =
            device.create_shader_module(include_wgsl!("shaders/debug_shader.wgsl"));

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

        let (image_w, image_h) = image::io::Reader::open("../assets/atlas.png")
            .unwrap()
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
        )?;
        register_table.register_block(
            2,
            BlockMeta {
                name: ResourceLocation::new("test_b"),
                ty: BlockType::Solid,
                atlas_coord: [atlas_meta.get(16, 6)?; 6],
            },
        )?;

        let chunk = Chunk::default();
        let render_chunk = RenderChunk::new(&device, &chunk, &register_table);

        let game = Game::default();
        let input_state = InputState::default();

        let settings = Settings {
            font: include_bytes!("../assets/fonts/Minecraft.otf"),
            font_size: 18.0,
        };

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

        let fps_text_section = Section::default()
            .add_text(
                Text::new("Hello World Test AAAAAAAAAAAAA")
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(25.0),
            )
            .with_layout(Layout::default().v_align(wgpu_text::glyph_brush::VerticalAlign::Center))
            .with_screen_position((50.0, config.height as f32 * 0.5))
            .to_owned();

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

            render_chunk,

            camera,
            matrix_uniform,

            brush,
            settings,
            fps: 0.0,
            dt_timer: Instant::now(),
            global_timer: Instant::now(),

            fps_text_section,

            input_state,
            game,

            register_table,
            debug_mode: false,
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
        // Time between this and the previous frame
        let delta_time = self.dt_timer.elapsed();
        // Set the timer to 0
        self.dt_timer = Instant::now();

        // Game Update
        self.game.update(&self.input_state);

        // FPS Text Update
        self.fps_text_section.text[0] = OwnedText::new(
            format!(
                "delta time: {}\nfps: {}",
                delta_time.as_secs_f32(),
                1.0 / delta_time.as_secs_f32()
            )
            .to_string(),
        )
        .with_z(0.0)
        .with_scale(25.0)
        .with_color([0.0, 0.0, 0.0, 1.0]);

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

        self.brush
            .queue(&self.device, &self.queue, vec![&self.fps_text_section])
            .unwrap();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Blockworld Render Pass"),
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

            render_pass.set_vertex_buffer(0, self.render_chunk.vertex_buffer.slice(..));
            render_pass.draw(0..self.render_chunk.vertex_count, 0..1);
            self.brush.draw(&mut render_pass);
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
        exec_instr_from_string(console_string, self).await?;
        Ok(())
    }
}

impl<'a> Debug for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("window", &self.window)
            .field("surface", &self.surface)
            .field("device", &self.device)
            .field("queue", &self.queue)
            .field("config", &self.config)
            .field("size", &self.size)
            .field("main_pipeline", &self.main_pipeline)
            .field("wireframe_pipeline", &self.wireframe_pipeline)
            .field("render_chunk", &self.render_chunk)
            .field("texture", &self.texture)
            .field("texture_bind_group", &self.texture_bind_group)
            .field("depth_texture", &self.depth_texture)
            .field("camera", &self.camera)
            .field("input_state", &self.input_state)
            .field("game", &self.game)
            .field("fps", &self.fps)
            .field("dt_timer", &self.dt_timer)
            .field("global_timer", &self.global_timer)
            .field("fps_text_section", &self.fps_text_section)
            .field("settings", &self.settings)
            .field("register_table", &self.register_table)
            .finish()
    }
}
