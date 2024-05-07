use suoi_types::Angle;

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

pub struct Projection<A>
where
    A: Angle,
{
    kind: ProjectionType,
    planes: ClippingPlanes,
    fov: A,
}

impl<A> Projection<A>
where
    A: Angle,
{
    pub fn new(kind: ProjectionType, planes: ClippingPlanes, fov: A) -> Self {
        Self { kind, planes, fov }
    }

    pub fn kind(&self) -> &ProjectionType {
        &self.kind
    }

    pub fn planes(&self) -> &ClippingPlanes {
        &self.planes
    }

    pub fn fov(&self) -> A {
        self.fov
    }
}
