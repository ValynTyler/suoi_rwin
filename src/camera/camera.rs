use suoi_types::Matrix;
use suoi_types::Matrix4;
use suoi_types::Transform;
use suoi_types::Vector;
use suoi_types::Vector3;

use crate::{Projection, ProjectionType, Screen};

#[allow(unused)]
pub struct Camera {
    pub projection: Projection,
    pub transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            projection: Default::default(),
            transform: Default::default(),
        }
    }
}

impl Camera {
    pub fn new(projection: Projection, transform: Transform) -> Self {
        Self {
            projection,
            transform,
        }
    }

    pub fn view_matrix(&self) -> Matrix4 {
        let u = self.transform.up();
        let r = self.transform.right();
        let f = self.transform.forward();

        let t = Vector3 {
            x: -r.dot(self.transform.position()),
            y: -u.dot(self.transform.position()),
            z: -f.dot(self.transform.position()),
        };

        Matrix4::look_at(f, u, r, t).transpose()
    }

    pub fn projection_matrix(&self, screen: &Screen) -> Matrix4 {
        match self.projection.kind() {
            ProjectionType::Perspective(fov) => Matrix4::perspective(
                fov.0,
                screen.aspect_ratio(),
                self.projection.planes().near,
                self.projection.planes().far,
            ),
            ProjectionType::Ortho(width, height) => Matrix4::ortho(
                -(*width as f32) / 2.0,
                (*width as f32) / 2.0,
                -(*height as f32) / 2.0,
                (*height as f32) / 2.0,
                self.projection.planes().near,
                self.projection.planes().far,
            ),
        }
    }

    pub fn inverse_projection_matrix(&self, screen: &Screen) -> Matrix4 {
        match self.projection.kind() {
            ProjectionType::Perspective(fov) => Matrix4::inverse_perspective(
                fov.0,
                screen.aspect_ratio(),
                self.projection.planes().near,
                self.projection.planes().far,
            ),
            ProjectionType::Ortho(_width, _height) => {
                todo!()
            }
        }
    }
}
