use nerd::matrix::Matrix4;
use nerd::vector::Vector3;
use suoi_types::Transform;
use suoi_types::Rad;
use suoi_types::Angle;

use crate::{Projection, ProjectionType, Screen};

#[allow(unused)]
pub struct Camera {
    pub projection: Projection<Rad>,
    pub transform: Transform,
}

impl Camera {
    pub fn new(projection: Projection<Rad>, transform: Transform) -> Self {
        Self {
            projection,
            transform,
        }
    }

    #[rustfmt::skip]
    pub fn view_matrix(&self) -> Matrix4 {
        let u = self.transform.up();
        let r = self.transform.right();
        let f = self.transform.forward();

        let t = Vector3 {
            x: -r.dot(self.transform.position()),
            y: -u.dot(self.transform.position()),
            z: -f.dot(self.transform.position()),
        };

        Matrix4([
             r.x,  u.x,  f.x, 0.,
             r.y,  u.y,  f.y, 0.,
             r.z,  u.z,  f.z, 0.,
            -t.x, -t.y, -t.z, 1.,
        ])
    }

    pub fn projection_matrix(&self, screen: &Screen) -> Matrix4 {
        match self.projection.kind() {
            ProjectionType::Perspective => nerd::matrix::Matrix4::perspective(
                self.projection.fov().deg().0,
                screen.aspect_ratio(),
                self.projection.planes().near,
                self.projection.planes().far,
            ),
            ProjectionType::Ortho => todo!(),
        }
    }
}
