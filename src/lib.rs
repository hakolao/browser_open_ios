use mobile_entry_point::mobile_entry_point;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[mobile_entry_point]
fn main() {
    let start_time = Instant::now();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(256.0, 256.0))
        .build(&event_loop)
        .unwrap();
    let mut opened = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
                if (Instant::now() - start_time).as_secs_f32() >= 2.0 {
                    let url = "https://www.google.com";
                    if !opened && webbrowser::open(url).is_ok() {
                        println!("Successfully opened {url}");
                        opened = true;
                    };
                }
            }
            _ => (),
        }
    });
}
