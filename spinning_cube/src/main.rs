use std::collections::HashSet;
use std::time::Instant;

use glutin::dpi::*;
use glutin::ContextTrait;

use cgmath::{ self, Deg, Point3, Vector3, Matrix4, InnerSpace };

mod model;
mod shader;

pub struct RenderContext {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    cam_loc: Point3<f32>,
}

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
        // Set the clear color to all black
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);

        // Enable depth testing
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        
        // Enable culling triangles not facing the camera
        gl::Enable(gl::CULL_FACE);
    }

    let camera_loc = Point3::new(4.0, 0.0, -3.0);
    let camera_up = Vector3::new(0.0, 1.0, 0.0);
    let camera_front = Vector3::new(0.0, 0.0, -1.0);

    let mut ctx = RenderContext {
        projection: cgmath::perspective(Deg(45.0), 4.0 / 3.0, 0.1, 100.0),
        view: Matrix4::look_at(
            camera_loc, // Camera location
            Point3::new(0.0, 0.0, 0.0), // Looking at origin
            Vector3::new(0.0, 1.0, 0.0), // Y is up
        ),
        cam_loc: camera_loc,
    };

    let mut keys = HashSet::new();

    let mut model = model::Model::new("assets/suzanne.obj", "assets/shader.vs", "assets/shader.fs")
        .expect("Couldn't create the model.");
    let mut rot_angle = 0.0;

    let mut last_frame = Instant::now();
    let mut delta_time;

    let mut running = true;
    while running {
        el.poll_events(|event| {
            use glutin::{ Event, WindowEvent, DeviceEvent, VirtualKeyCode, ElementState };

            match event {
                Event::WindowEvent{ event, .. } => match event {
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::Resized(logical_size) => {
                        let dpi_factor = win.get_hidpi_factor();
                        win.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => (),
                },
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::Key(key) => {
                        if let Some(k) = key.virtual_keycode {
                            match key.state {
                                ElementState::Pressed => keys.insert(k),
                                ElementState::Released => keys.remove(&k),
                            };
                        }
                    },
                    _ => (),   
                }
                _ => ()
            }
        });

        // Delta Time
        let current_frame = Instant::now();
        delta_time = (current_frame - last_frame).subsec_millis() as f32 / 1000.0;
        last_frame = current_frame;
        //

        /*
        let camera_speed = 2.5 * delta_time;

        if keys.contains(&glutin::VirtualKeyCode::W) {
            ctx.cam_loc += camera_speed * camera_front;
        }

        if keys.contains(&glutin::VirtualKeyCode::S) {
            ctx.cam_loc -= camera_speed * camera_front;
        }

        if keys.contains(&glutin::VirtualKeyCode::A) {
            ctx.cam_loc -= camera_front.cross(camera_up).normalize() * camera_speed;
        }

        if keys.contains(&glutin::VirtualKeyCode::D) {
            ctx.cam_loc += camera_front.cross(camera_up).normalize() * camera_speed;
        }

        ctx.view = Matrix4::look_at(
            ctx.cam_loc,
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        */

        unsafe {
            // Clear both the color and depth buffers
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        model.render(&ctx, || {
            // Rotate the model about the y axis
            rot_angle += 20.0 * delta_time;

            if rot_angle >= 360.0 {
                rot_angle = 0.0;
            }

            Matrix4::from_angle_y(Deg(rot_angle))
        });

        win.swap_buffers().expect("Could not swap window buffers");
    }
}
