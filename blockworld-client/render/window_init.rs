use crate::render::draw::{self, State};
use log::info;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowButtons, WindowId};

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
                self.render();
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

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = State::new(&event_loop).await;

    event_loop.run_app(&mut state).expect("Failed to run app.");
}
