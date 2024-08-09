use crate::get_cli_args;
use anyhow::*;
use clap::Parser;
use log::*;
use winit::*;

use super::render_state::RenderState;

struct WindowApplication {
    render_state: Option<RenderState>,
}

impl ApplicationHandler for WindowApplication {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let sensitivity = 0.002;
                self.camera.yaw -= delta.0 as f32 * sensitivity;
                self.camera.pitch -= delta.1 as f32 * sensitivity;
                if self.camera.pitch >= f32::to_radians(89.9) {
                    self.camera.pitch = f32::to_radians(89.9);
                } else if self.camera.pitch <= f32::to_radians(-89.9) {
                    self.camera.pitch = f32::to_radians(-89.9);
                }
            }
            _ => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.update();
                self.render().expect("Render Error!");
                // use inspect_err to avoid panic so that we can input instruction to display state to debug
                // self.try_exec_single_instr_from_console().inspect_err(
                //     |e| {
                //         error!("err when try_exec_single_instr_from_console {e:?}")
                //     }
                // );
                self.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.resize(size);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == KeyCode::Escape {
                    event_loop.exit();
                }
                self.input_state.handle_event(&event);
                let key = event.logical_key;

                // ! NOT IDEAL
                // ! FIX LATER
                if key == Key::Named(NamedKey::F1) && event.state == ElementState::Released {
                    self.debug_mode = !self.debug_mode;
                }
            }
            _ => (),
        }
    }
}

pub async fn run() -> Result<()> {
    env_logger::init();
    let boot_args = BootArgs::parse();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = Blockworld::new(&event_loop, &boot_args).await?;

    event_loop
        .run_app(&mut state)
        .with_context(|| format!("Failed to run app"))?;

    Ok(())
}