//! Window creation and winit event loop.
//!
//! Wires up `WindowApplication` (the `ApplicationHandler` impl) to
//! forward winit events into `RenderState`. Exports `run()` which is
//! the binary entry point called from `main.rs`.

use glam::vec2;
use log::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use super::render_state::RenderState;

/// The application struct.
///
/// Holds an `Option<RenderState>` because winit guarantees
/// `resumed()` is called before any other events, so `RenderState`
/// is always `Some` by the time events fire.
#[derive(Default)]
struct App {
    state: Option<RenderState>,
}

impl ApplicationHandler for App {
    /// Called once at startup. Creates the window and initializes wgpu.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title(blockworld_utils::GAME_NAME)
                    .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720)),
            )
            .expect("Failed to create window");

        // Lock cursor for FPS-style camera
        window
            .set_cursor_grab(winit::window::CursorGrabMode::Confined)
            .ok();
        window.set_cursor_visible(false);

        match RenderState::new(window) {
            Some(s) => self.state = Some(s),
            None => {
                error!("No GPU adapter found");
                event_loop.exit();
            }
        }
    }

    /// Raw device events (mouse motion for camera rotation).
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if let Some(s) = &mut self.state {
                s.world_renderer
                    .camera
                    .update_rotation(vec2(delta.0 as f32, delta.1 as f32));
            }
        }
    }

    /// Window-level events: close, resize, redraw, keyboard.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let s = match &mut self.state {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            // Continuous redraw loop via ControlFlow::Poll
            WindowEvent::RedrawRequested => {
                s.update();
                s.render().ok();
                s.window.request_redraw();
            }
            WindowEvent::Resized(size) => s.resize(size),
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == KeyCode::Escape {
                    event_loop.exit();
                }
                s.input_manager.handle_key_event(&event);

                if let PhysicalKey::Code(key) = event.physical_key {
                    if key == KeyCode::F1 && event.state == ElementState::Released {
                        s.world_renderer.debug_mode = !s.world_renderer.debug_mode;
                    }
                }
            }
            _ => {}
        }
    }
}

/// Entry point. Initializes logging, creates the event loop, and blocks.
/// Uses `ControlFlow::Poll` for continuous rendering (max FPS).
pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::default()).ok();
}
