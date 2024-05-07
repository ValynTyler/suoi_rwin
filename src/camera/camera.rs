use nerd::matrix::Matrix4;
use suoi_types::Transform;

use crate::{Projection, ProjectionType, Screen};

#[allow(unused)]
pub struct Camera {
    pub projection: Projection,
    pub transform: Transform,
}

impl Camera {
    pub fn new(projection: Projection, transform: Transform) -> Self {
        Self {
            projection,
            transform,
        }
    }

    #[rustfmt::skip]
    pub fn view_matrix(&self) -> Matrix4 {
        // Matrix4::look_at(
        //     self.local_space.origin,
        //     self.local_space.origin + self.local_space.forward,
        //     self.local_space.up,
        // )

        let f = self.transform.forward();
        let r = self.transform.right();
        let u = self.transform.up();

        let t = self.transform.position();

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
                self.projection.fov().deg(),
                screen.aspect_ratio(),
                self.projection.planes().near,
                self.projection.planes().far,
            ),
            ProjectionType::Ortho => todo!(),
        }
    }
}
