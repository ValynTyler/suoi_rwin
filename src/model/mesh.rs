use std::ffi::c_void;

use gl::types::*;
use suoi_simp::obj_mesh::ObjMesh;
use suoi_types::Vector;

use crate::{GraphicsObject, Texture, Vertex, SIZE_OF_FLOAT};

#[allow(unused)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub textures: Vec<Texture>,
    vao: GLuint,
    vbo: GLuint,
}

impl From<&ObjMesh> for Mesh {
    fn from(value: &ObjMesh) -> Self {
        let mut vertices = Vec::<Vertex>::new();

        for face in value.faces() {
            for element in face.elements() {
                vertices.push(Vertex {
                    position: value.positions()
                        [(element.position_index() - value.min_pos_index()) as usize],
                    normal: value.normals()
                        [(element.normal_index() - value.min_nrm_index()) as usize],
                    uv: value.uvs()[(element.uv_index() - value.min_uvs_index()) as usize],
                });
            }
        }

        unsafe { Mesh::new(vertices, vec![]) }
    }
}

impl GraphicsObject for Mesh {
    /// calls `f` with self.vao as the bound `ArrayObject`
    unsafe fn with<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        // bind the vertex array object (VAO) as the current vertex array in order to work with it
        gl::BindVertexArray(self.vao);

        // call `f`
        f();

        // Unbind
        gl::BindVertexArray(0);
    }
}

impl Mesh {
    // Creates a new instance of `Mesh` and loads it
    pub unsafe fn new(vertices: Vec<Vertex>, textures: Vec<Texture>) -> Self {
        // initialize objects
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        // give each object a unique identifier
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        // initialize and load mesh
        let mesh = Self {
            vao,
            vbo,
            vertices,
            textures,
        };
        mesh.load();

        mesh
    }

    pub unsafe fn load(&self) {
        self.with(|| {
            // bind data to the active buffers (VBO, EBO)
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertex_buffer().len() * crate::SIZE_OF_FLOAT as usize) as GLsizeiptr,
                &self.vertex_buffer()[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = (3 + 3 + 2) * SIZE_OF_FLOAT;

            let pos_addr = 0;
            let col_addr = 1;
            let tex_addr = 2;

            // positions
            Self::vertex_attrib(pos_addr, 3, stride, 0);
            // normals
            Self::vertex_attrib(col_addr, 3, stride, 3);
            // uvs
            Self::vertex_attrib(tex_addr, 2, stride, 3 + 3);
        })
    }

    pub unsafe fn draw(&self) {
        self.with(|| {
            gl::ActiveTexture(gl::TEXTURE1);
            match self.textures.last() {
                Some(tex) => {
                    tex.with(|| gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32))
                }
                None => gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32),
            }
        })
    }

    pub fn vertex_buffer(&self) -> Vec<f32> {
        let mut buffer = vec![];

        for vertex in &self.vertices {
            buffer.append(&mut vertex.position.list());
            buffer.append(&mut vertex.normal.list());
            buffer.append(&mut vertex.uv.list());
        }

        buffer
    }

    /**
    # Mesh::vertex_attrib
    loads the correct OpenGL vertex attribute pointers
    ---
    address: the shader address correspondint to the destination of the data
    size: number of floating point values that compose current vertex attrib
    stride: total size of vertex in bytes
    offset: number of floating point values that precede the current attrib
    */
    unsafe fn vertex_attrib(address: GLuint, size: GLint, stride: GLsizei, offset: i32) {
        gl::EnableVertexAttribArray(address);
        gl::VertexAttribPointer(
            address,
            size,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (offset * SIZE_OF_FLOAT) as *const c_void,
        );
    }
}
