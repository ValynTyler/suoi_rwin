use std::path::PathBuf;

use suoi_simp::Obj;

use crate::{Mesh, Texture};

#[allow(unused)]
pub struct Model {
    path: PathBuf,
    meshes: Vec<Mesh>,
}

impl From<Obj> for Model {
    fn from(value: Obj) -> Self {
        let mut model = Self {
            path: value.path().to_owned(),
            meshes: vec![],
        };

        for obj_mesh in value.meshes() {
            let mut mesh: Mesh = obj_mesh.into();

            match obj_mesh.get_material().get_diffuse_path() {
                None => (),
                Some(path) => unsafe {
                    let tex_path = value.path().parent().unwrap().join(path);
                    mesh.textures.push(Texture::new(&tex_path));
                },
            };

            model.meshes.push(mesh);
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

    // pub fn model_matrix(&self) -> Matrix4 {
    //     &Matrix4::translate(self.transform.position()) * &self.transform.rotation().mat()
    // }
}
