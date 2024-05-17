use std::{ffi::c_void, fs::read_to_string};

use gl::types::*;
use suoi_types::{Matrix, Matrix4, Vector, Vector3};

use crate::{Camera, GraphicsObject, Screen, Shader, ShaderStage, SIZE_OF_FLOAT};

pub struct Line {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    start: Vector3,
    end: Vector3,
}

impl Line {
    pub unsafe fn new(start: Vector3, end: Vector3) -> Self {
        // initialize objects
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        // give each object a unique identifier
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        let vertex_source = &read_to_string("assets/shaders/line.vert").unwrap();
        let fragment_source = &read_to_string("assets/shaders/line.frag").unwrap();

        let shader = Shader::compile(
            ShaderStage::compile(&vertex_source, crate::ShaderStageType::Vertex).unwrap(),
            ShaderStage::compile(&fragment_source, crate::ShaderStageType::Fragment).unwrap(),
        )
        .unwrap();

        let line = Self {
            shader,
            vao,
            vbo,
            start,
            end,
        };
        line.load();

        line
    }

    pub unsafe fn load(&self) {
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (self.vertex_buffer().len() * crate::SIZE_OF_FLOAT as usize) as GLsizeiptr,
            &self.vertex_buffer()[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * SIZE_OF_FLOAT,
            0 as *const c_void,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    pub unsafe fn draw(&self, camera: &Camera, screen: &Screen) {
        self.shader.with(|| {
            self.shader.set_uniform("model", &Matrix4::identity());
            self.shader.set_uniform("view", &camera.view_matrix());
            self.shader
                .set_uniform("projection", &camera.projection_matrix(&screen).transpose());

            // uniform set color

            // bind the vertex array object (VAO) as the current vertex array in order to work with it
            gl::BindVertexArray(self.vao);

            gl::DrawArrays(gl::LINES, 0, 2);

            // Unbind
            gl::BindVertexArray(0);
        })
    }

    fn vertex_buffer(&self) -> Vec<f32> {
        let mut vertices = vec![];
        vertices.append(&mut self.start.list());
        vertices.append(&mut self.end.list());

        vertices
    }
    
    pub fn set_start(&mut self, start: Vector3) {
        self.start = start;
        unsafe { self.load() };
    }
    
    pub fn set_end(&mut self, end: Vector3) {
        self.end = end;
        unsafe { self.load() };
    }
}
