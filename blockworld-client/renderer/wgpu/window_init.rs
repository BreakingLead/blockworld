use std::process::exit;

use anyhow::Context;
use application::ApplicationHandler;
use event::WindowEvent;
use event_loop::{ActiveEventLoop, EventLoop};
use keyboard::KeyCode;
use log::*;
use window::{Window, WindowId};
use winit::*;

use super::render_state::RenderState;

/// The main struct for window initialization and event handling.
#[derive(Default)]
struct WindowApplication {
    render_state: Option<RenderState>,
}

impl WindowApplication {
    /// Get the render state for the window.
    fn render_state_mut(&mut self) -> &mut RenderState {
        self.render_state.as_mut().unwrap()
    }

    fn render_state(&self) -> &RenderState {
        self.render_state.as_ref().unwrap()
    }
}

impl ApplicationHandler for WindowApplication {
    /// Initialize the application.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title(blockworld_utils::GAME_NAME));
        match window {
            Ok(window) => {
                self.render_state = Some(RenderState::new(window));
            }
            Err(_) => {
                error!("Failed to create window");
                self.exiting(event_loop);
                exit(-1);
            }
        }
    }

    /// Process a device event.
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.render_state_mut()
            .input_manager
            .handle_device_event(&event);
    }

    /// Process a window event.
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
                self.render_state_mut().update();
                self.render_state_mut().render();
                // use inspect_err to avoid panic so that we can input instruction to display state to debug
                // self.try_exec_single_instr_from_console().inspect_err(
                //     |e| {
                //         error!("err when try_exec_single_instr_from_console {e:?}")
                //     }
                // );
                self.render_state().window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.render_state_mut().resize(size);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == KeyCode::Escape {
                    event_loop.exit();
                }
                self.render_state_mut()
                    .input_manager
                    .handle_key_event(&event);

                let key = event.logical_key;

                // ! NOT IDEAL
                // ! FIX LATER
                if key == keyboard::Key::Named(keyboard::NamedKey::F1)
                    && event.state == event::ElementState::Released
                {
                    self.render_state_mut().world_renderer.debug_mode =
                        !self.render_state().world_renderer.debug_mode;
                }
            }
            _ => (),
        }
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);

    let mut state = WindowApplication::default();

    event_loop
        .run_app(&mut state)
        .with_context(|| "Failed to run app".to_string())
        .unwrap();
}
