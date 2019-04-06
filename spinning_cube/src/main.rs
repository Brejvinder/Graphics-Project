use glutin::dpi::*;
use glutin::ContextTrait;

fn main() {
    let mut el = glutin::EventsLoop::new();

    let win = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed({
            glutin::WindowBuilder::new()
                .with_title("COMP3520 Spinning Cube")
                .with_dimensions(LogicalSize::new(1024.0, 768.0))
        }, &el)
        .expect("Couldn't build glutin context");

    unsafe {
        win.make_current().expect("Couldn't make window current context");

        // Load OpenGL symbols
        gl::load_with(|s| win.get_proc_address(s) as *const _);
    }
    
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        el.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = win.get_hidpi_factor();
                        win.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        win.swap_buffers().expect("Could not swap window buffers");
    }
}
