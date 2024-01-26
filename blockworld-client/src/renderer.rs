use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Blockworld")
        .build(&event_loop)
        .unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => match event.logical_key {
                winit::keyboard::Key::Named(c) => {}
                winit::keyboard::Key::Character(c) => {
                    if c == "a" {
                        println!("wtf");
                    }
                }
                _ => (),
            },
            _ => (),
        },
        Event::LoopExiting => {
            println!("Save...");
        }
        _ => (),
    });
}