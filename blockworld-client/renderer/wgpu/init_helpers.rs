use crate::get_cli_args;
use pollster::FutureExt;
use wgpu::*;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Fullscreen, Window},
};

/// Create the window instance for creating the surface of `wgpu`.
pub fn create_window(event_loop: &EventLoop<()>) -> Window {
    let mut window_attrs = Window::default_attributes().with_title("Blockworld Indev");
    let args = get_cli_args();
    // set screen size based on boot_args
    if args.full_screen {
        window_attrs = window_attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
    } else {
        window_attrs = window_attrs.with_inner_size(PhysicalSize::new(args.width, args.height))
    }
    let window = event_loop.create_window(window_attrs).unwrap();
    window
        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
        .unwrap();
    window.set_cursor_visible(false);

    window
}

/// Create the `wgpu` instance.
pub fn create_instance() -> Instance {
    Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    })
}

/// Create the `wgpu` surface from the window.
pub fn create_surface<'window>(
    instance: &Instance,
    window: &'window Window,
) -> Result<Surface<'window>, wgpu::CreateSurfaceError> {
    instance.create_surface(window)
}

/// Create the `wgpu` adapter from the instance and surface.
pub fn create_adapter(instance: &Instance, surface: &Surface<'_>) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .block_on()
        .unwrap()
}

/// Create the `wgpu` device and queue from the adapter.
pub fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::POLYGON_MODE_LINE,
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .block_on()
        .unwrap();
    (device, queue)
}

/// Create the `wgpu` surface configuration from the window size and surface.
pub fn create_surface_config(
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
