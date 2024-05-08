use suoi_types::{Angle, Rad};

#[allow(unused)]
pub struct ClippingPlanes {
    pub(crate) near: f32,
    pub(crate) far: f32,
}

impl ClippingPlanes {
    pub fn new(near: f32, far: f32) -> Self {
        Self { near, far }
    }
}

pub enum ProjectionType {
    Ortho,
    Perspective,
}

pub struct Projection {
    kind: ProjectionType,
    planes: ClippingPlanes,
    fov: Rad,
}

impl Projection {
    pub fn new<A>(kind: ProjectionType, planes: ClippingPlanes, fov: A) -> Self
    where
        A: Angle,
    {
        Self { kind, planes, fov: fov.rad() }
    }

    pub fn kind(&self) -> &ProjectionType {
        &self.kind
    }

    pub fn planes(&self) -> &ClippingPlanes {
        &self.planes
    }

    pub fn fov(&self) -> Rad {
        self.fov
    }
}
