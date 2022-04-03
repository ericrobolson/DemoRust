use core_connection::{dev_connection::Client, PacketBuffer};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let mut conn = Client::new("testyMcTest", 60);
    let mut timer = core_time::Timer::new(60);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        if timer.tick().triggered {
            while let Some(packet) = conn.poll() {
                let text = String::from_utf8(packet.data().to_vec()).unwrap();
                println!("Received '{}'", text);
            }

            if let Some(packet) = PacketBuffer::from_str("Hello world!") {
                conn.send(packet);
            }
        }
    });
}
