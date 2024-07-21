use std::fmt::Debug;
use std::marker::PhantomData;
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
use super::render_array::RenderArray;
use super::render_system::RenderSystem;
use super::texture::Texture;
use super::{
    pipeline::{RegularPipeline, WireframePipeline},
    render_chunk::RenderChunk,
};
use super::{uniform::*, window_init};

/// state contains all things the game needs
pub struct State<'state> {
    pub window: Window,

    // IO
    pub input_state: InputState,

    // The Game
    pub game: Game,
    pub fps: f32,
    pub dt_timer: Instant,
    pub global_timer: Instant,

    // Settings
    pub settings: Settings,

    pub register_table: RegisterTable,

    pub render_system: RenderSystem<'state>,

    // Debug
    pub debug_mode: bool,
}

impl<'a> State<'a> {
    pub async fn new(event_loop: &EventLoop<()>, boot_args: &BootArgs) -> Result<State<'a>> {
        let window = window_init();
        let _player_state: PlayerState = Default::default();

        // Camera thingy
        let camera = Camera::new(size.width as f32 / size.height as f32);
        // Texture & its bind group

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

        let mut game = Game::default();
        let input_state = InputState::default();

        let settings = Settings {
            font: include_bytes!("../assets/fonts/Minecraft.otf"),
            font_size: 18.0,
        };

        Ok(Self {
            window,

            camera,

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

    pub fn update(&mut self) {
        // Time between this and the previous frame
        let delta_time = self.dt_timer.elapsed();
        // Set the timer to 0
        self.dt_timer = Instant::now();

        // Game Update
        self.game.update(&self.input_state);

        // FPS Text Update
        // self.fps_text_section.text[0] = OwnedText::new(
        //     format!(
        //         "delta time: {}\nfps: {}",
        //         delta_time.as_secs_f32(),
        //         1.0 / delta_time.as_secs_f32()
        //     )
        //     .to_string(),
        // )
        // .with_z(0.0)
        // .with_scale(25.0)
        // .with_color([0.0, 0.0, 0.0, 1.0]);

        // self.window.set_title(
        //     format!(
        //         "Blockworld Dev [fps: {:.0}]",
        //         1.0 / delta_time.as_secs_f32()
        //     )
        //     .as_str(),
        // );

        // Camera Update

        self.camera.update(&self.game.player_state);

        self.matrix_uniform.uniform.update_matrix(&self.camera);
        self.queue.write_buffer(
            &self.matrix_uniform.buffer,
            0,
            bytemuck::cast_slice(&[*self.matrix_uniform.uniform]),
        );
    }

    pub fn render(&mut self) -> Result<()> {
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

            // draw all chunks
            for chunk in self.render_array.chunks() {
                render_pass.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
                render_pass.draw(0..chunk.vertex_count, 0..1);
            }
            // self.brush.draw(&mut render_pass);
        }

        self.render_system
            .queue
            .submit(std::iter::once(encoder.finish()));
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
        todo!()
    }
}

fn init_window(boot_args: &BootArgs, event_loop: EventLoop<()>) -> Result<Window> {
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

    Ok(window)
}
