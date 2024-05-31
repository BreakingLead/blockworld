use crate::render::draw::State;
use crate::BootArgs;
use anyhow::*;
use clap::Parser;
use log::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::WindowId;

impl<'a> ApplicationHandler for State<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resumed!");
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
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
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
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

    let mut state = State::new(&event_loop, &boot_args).await?;

    event_loop
        .run_app(&mut state)
        .with_context(|| format!("Failed to run app"))?;

    Ok(())
}
