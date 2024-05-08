pub mod vertex;

use suoi_simp::Obj;
pub use vertex::*;

pub mod mesh;
pub use mesh::*;

use gl::types::*;

pub const SIZE_OF_FLOAT: GLsizei = std::mem::size_of::<GLfloat>() as GLsizei;

#[allow(unused)]
pub struct Model {
    meshes: Vec<Mesh>,
}

impl From<Obj> for Model {
    fn from(value: Obj) -> Self {
        let mut model = Self { meshes: vec![] };
        
        for mesh in value.meshes() {
            model.meshes.push(mesh.into());
        }

        model
    }
}

impl Model {
    pub unsafe fn draw(&self) {
        for mesh in &self.meshes {
            mesh.draw()
        }
    }
}
