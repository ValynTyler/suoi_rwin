use suoi_simp::Obj;
use suoi_types::{Matrix4, Transform};

use crate::Mesh;

pub struct Model {
    meshes: Vec<Mesh>,
    transform: Transform,
}

impl From<Obj> for Model {
    fn from(value: Obj) -> Self {
        let mut model = Self { meshes: vec![], transform: Transform::default() };
        
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

    pub fn model_matrix(&self) -> Matrix4 {
        Matrix4::translate(self.transform.position())
    }
}
