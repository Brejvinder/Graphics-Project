use std::{ mem, ptr, path::Path, io::BufReader, fs::File };

use cgmath::{ Matrix, Matrix4 };
use gl;
use obj::{ self, Vertex };

use crate::{ RenderContext, shader::ShaderProgram };

pub struct Model {
    // The vertex array
    vertex_array: gl::types::GLuint,
    
    // The buffers
    vertex_buffer: gl::types::GLuint,
    normal_buffer: gl::types::GLuint,
    index_buffer: gl::types::GLuint,

    // The shader program
    program: ShaderProgram,
    mvp_location: gl::types::GLint,
    m_location: gl::types::GLint,
    v_location: gl::types::GLint,
    light_location: gl::types::GLint,

    num_indices: i32,
}

impl Model {
    pub fn new<P, V, F>(p: P, v: V, f: F) -> Result<Self, String>
        where P: AsRef<Path>,
              V: AsRef<str>,
              F: AsRef<str>
    {
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

        unsafe {
            let program = ShaderProgram::load_from(v.as_ref(), f.as_ref())?;
            let mvp_location = program.get_uniform("MVP");
            let m_location = program.get_uniform("M");
            let v_location = program.get_uniform("V");
            let light_location = program.get_uniform("LightPosition_worldspace");

            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);
            
            let mut vertex_buffer = 0;
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<f32>()) as _, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

            let mut normal_buffer = 0;
            gl::GenBuffers(1, &mut normal_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (normals.len() * mem::size_of::<f32>()) as _, normals.as_ptr() as *const _, gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

            let mut index_buffer = 0;
            gl::GenBuffers(1, &mut index_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<u16>()) as _, indices.as_ptr() as *const _, gl::STATIC_DRAW);

            Ok(Model {
                vertex_array: vertex_array,
                
                vertex_buffer: vertex_buffer,
                normal_buffer: normal_buffer,
                index_buffer: index_buffer,

                program: program,
                mvp_location: mvp_location,
                m_location: m_location,
                v_location: v_location,
                light_location: light_location,

                num_indices: indices.len() as _,
            })
        }
    }

    pub fn render<F>(&mut self, ctx: &RenderContext, f: F)
        where F: FnOnce() -> Matrix4<f32>
    {
        let model = f();
        let mvp = ctx.projection * ctx.view * model;

        unsafe {
            self.program.use_program();
            gl::UniformMatrix4fv(self.mvp_location, 1, gl::FALSE, mvp.as_ptr() as *const _);
            gl::UniformMatrix4fv(self.m_location, 1, gl::FALSE, model.as_ptr() as *const _);
            gl::UniformMatrix4fv(self.v_location, 1, gl::FALSE, ctx.view.as_ptr() as *const _);
            gl::Uniform3f(self.light_location, 4.0, 4.0, 4.0); // Light location is at (4, 4, 4)

            gl::BindVertexArray(self.vertex_array);
            gl::DrawElements(gl::TRIANGLES, self.num_indices, gl::UNSIGNED_SHORT, ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}

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