use std::{ mem, ptr };

use cgmath::{ Deg, Matrix, SquareMatrix, Matrix4 };
use gl;

use crate::{ RenderContext, shader::ShaderProgram };

static VERTEX_DATA: &[f32] = &[
    -1.0,-1.0,-1.0,
    -1.0,-1.0, 1.0,
    -1.0, 1.0, 1.0,
    1.0, 1.0,-1.0,
    -1.0,-1.0,-1.0,
    -1.0, 1.0,-1.0,
    1.0,-1.0, 1.0,
    -1.0,-1.0,-1.0,
    1.0,-1.0,-1.0,
    1.0, 1.0,-1.0,
    1.0,-1.0,-1.0,
    -1.0,-1.0,-1.0,
    -1.0,-1.0,-1.0,
    -1.0, 1.0, 1.0,
    -1.0, 1.0,-1.0,
    1.0,-1.0, 1.0,
    -1.0,-1.0, 1.0,
    -1.0,-1.0,-1.0,
    -1.0, 1.0, 1.0,
    -1.0,-1.0, 1.0,
    1.0,-1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0,-1.0,-1.0,
    1.0, 1.0,-1.0,
    1.0,-1.0,-1.0,
    1.0, 1.0, 1.0,
    1.0,-1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 1.0,-1.0,
    -1.0, 1.0,-1.0,
    1.0, 1.0, 1.0,
    -1.0, 1.0,-1.0,
    -1.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
    -1.0, 1.0, 1.0,
    1.0,-1.0, 1.0
];

pub struct Cube {
    vertex_array: gl::types::GLuint,
    vertex_buffer: gl::types::GLuint,
    program: ShaderProgram,
    mvp_location: gl::types::GLint,
    rot_angle: f32,
}

impl Cube {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let program = ShaderProgram::load_from("assets/shader.vs", "assets/shader.fs")?;
            let mvp = program.get_uniform("MVP");

            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);
            
            let mut vertex_buffer = 0;
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(VERTEX_DATA) as _, VERTEX_DATA.as_ptr() as *const _, gl::STATIC_DRAW);

            Ok(Cube {
                vertex_array: vertex_array,
                vertex_buffer: vertex_buffer,
                program: program,
                mvp_location: mvp,
                rot_angle: 0.0,
            })
        }
    }

    pub fn render(&mut self, ctx: &RenderContext, _delta: f32) {
        // For now we'll just increase the angle.
        self.rot_angle += 0.5;

        if self.rot_angle >= 360.0 {
            self.rot_angle = 0.0;
        }

        let model = Matrix4::from_angle_y(Deg(self.rot_angle));
        let mvp = ctx.projection * ctx.view * model;

        unsafe {
            self.program.use_program();
            gl::UniformMatrix4fv(self.mvp_location, 1, gl::FALSE, mvp.as_ptr() as *const _);

            // Enable the vertex data in the shader.
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

            gl::DrawArrays(gl::TRIANGLES, 0, 12 * 3);

            gl::DisableVertexAttribArray(0);
        }
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        unsafe {
            // Cleanup OpenGL stuff
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteVertexArrays(1, &self.vertex_array);
        }
    }
}