use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Blockworld")
            .build(&event_loop)
            .unwrap(),
    );
    window.set_cursor_visible(false);
    window
        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
        .unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = blockworld_client::State::new(Arc::clone(&window)).await;

    let mut input_helper = WinitInputHelper::new();

    let _ = event_loop.run(move |event, elwt| {
        if input_helper.update(&event) {
            if input_helper.close_requested() {
                elwt.exit();
                return;
            }
            if let Some(new_size) = input_helper.window_resized() {
                log::info!("Resized window");
                state.resize(new_size);
            }

            state.input(&input_helper);

            state.update(input_helper.delta_time().unwrap());

            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if it's lost or outdated
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    state.resize(state.size)
                }
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    elwt.exit();
                }
                // We're ignoring timeouts
                Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
            }
        }
    });
}
