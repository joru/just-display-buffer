use std::num::NonZeroU32;
use winit::event::Event;

pub fn just_display_it(redraw_callback: impl Fn(u32, u32, &mut [u32]) + 'static) {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_maximized(true)
        .with_title("Hi!")
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE)
        .build(&event_loop)
        .unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        //control_flow.set_wait();
        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                println!("Done.");
                control_flow.set_exit();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let size = window.inner_size();
                let (width, height) = (size.width, size.height);
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                let mut buffer = surface.buffer_mut().unwrap();
                let pixels = buffer.as_mut();

                redraw_callback(width, height, pixels);

                buffer.present().unwrap();
            }
            _ => (),
        }
    });
}
