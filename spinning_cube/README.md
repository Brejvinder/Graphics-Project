# Spinning Model

![Screenshot](assets/screenshot.png)

## Breakdown

This project is built with Rust and follows the structure of a Rust project.

- `assets/` includes stuff such as the models and shaders loaded by the program
- `src/` includes the project source
- `Cargo.lock` lock file maintained by Cargo
- `Cargo.toml` configuration file for Cargo, contains things such as the dependencies

The dependency crates used in this project (from https://crates.io/) are as follows:

- `glutin` is a cross platform OpenGL context and window creator
- `gl` are Rust bindings to OpenGL
- `cgmath` is a linear algebra and math library
- `obj-rs` is a Wavefront obj parser library

## Building

Building this project is extremely easy. The only requirement is the latest stable installation of Rust (https://rust-lang.org/).

Once Rust is installed the project can be built with the command `cargo run` from the project directory.

## Code

### Shader

Two wrapper types (`shader::Shader` and `shader::ShaderProgram`) were created to simplify the loading and unloading of shaders. Errors encountered during the shader compilation and linking stages of shader creation are converted to Rust strings for display.

For example, in the `shader::ShaderProgram` type, link errors are converted as such:

```rust
// Get the length of the info log
let mut len = 0;
gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

// If the info log length is set there has been an error
let res = if len > 0 {
    // Buffer to put the info log in and get the info log from OpenGL
    let mut buf = vec![0; len as usize];
    gl::GetProgramInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut _);

    // Convert the info log into a Rust string
    let s = CStr::from_bytes_with_nul(&buf)
        .expect("Shader info log has malformed nul")
        .to_string_lossy()
        .to_string();

    // Delete the program as it did not have successful creation
    gl::DeleteProgram(id);

    Err(s)
} else {
    // The info log is not set therefore the shader program has successfully linked
    Ok(ShaderProgram(id))
};
```

A similar snippet of code is implemented in `shader::Shader` to check compilation status of the shader.

Both `shader::Shader` and `shader::ShaderProgram` implement the Rust `Drop` trait to automatically handle deletion when falling out of scope.

For example, for the `shader::Shader` type the drop implementation is:
```rust
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}
```

### Model

A generic Model type has been created that can load a specified Wavefront .obj file, vertex shader, and fragment shader. The crate `obj-rs` is used to load the model data as follows:

```rust
// Load the obj file data from the specified file if it exists.
let file = File::open(p.as_ref()).map_err(|e| format!("{}", e))?;
let obj = obj::load_obj(BufReader::new(file)).map_err(|e| format!("{}", e))?;

let input_verts: Vec<Vertex> = obj.vertices;

let mut vertices: Vec<f32> = Vec::new();
let mut normals: Vec<f32> = Vec::new();
let indices: Vec<u16> = obj.indices;

for vert in input_verts {
    vertices.extend(&vert.position);
    normals.extend(&vert.normal);
}
```

Now that we have the model data we want to try and load the supplied shaders passed using our helpful wrapper with `let program = ShaderProgram::load_from(v.as_ref(), f.as_ref())?;`. We then load the uniform locations we will use later when rendering and move on to uploading the data to the buffers. For the vertex array this is done like so:

```rust
let mut vertex_buffer = 0;
gl::GenBuffers(1, &mut vertex_buffer);
gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<f32>()) as _, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

gl::EnableVertexAttribArray(0);
gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
```

The `render` method on Model takes a `RenderContext` and a generic `FnOnce() -> Matrix4<f32>` type that can be a function pointer or a closure that provides the model matrix. This allows the model matrix to be created outside of the `Model` type making it more generic.

For example,

```rust
// Takes a closure with no parameters and returns the identity matrix.
model.render(&ctx, || Matrix4::identity());
```

This render method is responsible for setting the shader uniforms, binding the vertex array, and drawing the triangles set previously.

```rust
unsafe {
    // Use the program and set the uniforms
    self.program.use_program();
    gl::UniformMatrix4fv(self.mvp_location, 1, gl::FALSE, mvp.as_ptr() as *const _);
    gl::UniformMatrix4fv(self.m_location, 1, gl::FALSE, model.as_ptr() as *const _);
    gl::UniformMatrix4fv(self.v_location, 1, gl::FALSE, ctx.view.as_ptr() as *const _);
    gl::Uniform3f(self.light_location, 4.0, 4.0, 4.0); // Light location is at (4, 4, 4)
    
    // Bind the vertex array and draw the triangles
    gl::BindVertexArray(self.vertex_array);
    gl::DrawElements(gl::TRIANGLES, self.num_indices, gl::UNSIGNED_SHORT, ptr::null());
    gl::BindVertexArray(0);
    gl::UseProgram(0);
}
```

Model also contains a `Drop` implementation to clean up the buffers and the vertex array. It is not nessassary to clean up the `shader::ShaderProgram` because it has its own `Drop` implementation that cleans itself up.

```rust
impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            // Cleanup OpenGL stuff
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteBuffers(1, &self.normal_buffer);
            gl::DeleteBuffers(1, &self.index_buffer);
            gl::DeleteVertexArrays(1, &self.vertex_array);
        }
    }
}
```

### Main

Creating a window and OpenGL context with the crate `glutin` is simple and uses the builder pattern:

```rust
let win = glutin::ContextBuilder::new()
    .with_vsync(true)
    .build_windowed({
        glutin::WindowBuilder::new()
            .with_title("COMP3520 Spinning Cube")
            .with_dimensions(LogicalSize::new(1024.0, 768.0))
    }, &el)
    .expect("Couldn't build glutin context");
```

After window creation all that is needed to start drawing with OpenGL is the following:

```rust
unsafe {
    win.make_current().expect("Couldn't make window current context");

    // Load OpenGL symbols
    gl::load_with(|s| win.get_proc_address(s) as *const _);
}
```

We make this thread and Window the current OpenGL context and set the `gl` crate to use glutin for loading `OpenGL` symbols. `unsafe {}` is required because we are using FFI with OpenGL and Rust cannot guarantee anything about FFI code and raw pointers.

We set some initial OpenGL settings with:
```rust
unsafe {
    // Set the clear color to all black
    gl::ClearColor(0.0, 0.0, 0.0, 1.0);

    // Enable depth testing
    gl::Enable(gl::DEPTH_TEST);
    gl::DepthFunc(gl::LESS);
    
    // Enable culling triangles not facing the camera
    gl::Enable(gl::CULL_FACE);
}
```

We create the initial project matrix and view matrix and put it in a `RenderContext` so it can be easily sent to the `Model` render method.

```rust
let mut ctx = RenderContext {
    projection: cgmath::perspective(Deg(45.0), 4.0 / 3.0, 0.1, 100.0),
    view: Matrix4::look_at(
        camera_loc, // Camera location
        Point3::new(0.0, 0.0, 0.0), // Looking at origin
        Vector3::new(0.0, 1.0, 0.0), // Y is up
    ),
    cam_loc: camera_loc,
};
```

Then we load our model with the `Model` type we defined:
```rust
let mut model = model::Model::new("assets/suzanne.obj", "assets/shader.vs", "assets/shader.fs")
    .expect("Couldn't create the model.");
```

In the main render loop we first clear the color and depth buffer:
```rust
unsafe {
    // Clear both the color and depth buffers
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
}
```

and then render our model by calling its `render` method. 

```rust
model.render(&ctx, || {
    // Rotate the model about the y axis
    rot_angle += 20.0 * delta_time;

    if rot_angle >= 360.0 {
        rot_angle = 0.0;
    }

    Matrix4::from_angle_y(Deg(rot_angle))
});
```

Here we are rotating the model about the y axis by a constant amount each frame. The variable `rot_angle` is a local variable in the main program scope and can be modified and used inside of the closure passed to `render`. 

## Planned Features

- `Camera movement` - Some intial work has been completed for camera movement, we capture the key press events and store the currently pressed keys in a HashSet so we can check if a key is pressed for camera movement.
- `Texture loading` - Currently loaded models with textures do not upload and use UV values from the model. We would have to load the image data and upload it as a OpenGL texture as well as the vertex texture data to its own buffer for use in the shader.
- `Post processing` - Post processing shaders would have a fun idea to get to, we would need to create a pipeline for post processing that uses framebuffers and shaders to transform the data and finally render a completed image to the screen.