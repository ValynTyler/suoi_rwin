use nerd::matrix::Matrix4;

use crate::Screen;

pub enum ProjectionType {
    Ortho,
    Perspective,
}

pub struct ClippingPlanes {
    near: f32,
    far: f32,
}

impl ClippingPlanes {
    pub fn new(near: f32, far: f32) -> Self {
        Self { near, far }
    }
}

pub struct Camera {
    projection_type: ProjectionType,
    clipping_planes: ClippingPlanes,
    fov: f32,
}

impl Camera {
    pub fn new(projection: ProjectionType, clipping: ClippingPlanes, fov: f32) -> Self {
        Self {
            projection_type: projection,
            clipping_planes: clipping,
            fov,
        }
    }

    pub fn projection_matrix(&self, screen: &Screen) -> Matrix4 {
        match self.projection_type {
            ProjectionType::Perspective => nerd::matrix::Matrix4::perspective(
                self.fov,
                screen.aspect_ratio(),
                self.clipping_planes.near,
                self.clipping_planes.far,
            ),
            ProjectionType::Ortho => todo!(),
        }
    }
}
