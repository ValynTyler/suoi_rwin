use suoi_types::Deg;

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
    Ortho(u32, u32),
    Perspective(Deg),
}

pub struct Projection {
    kind: ProjectionType,
    planes: ClippingPlanes,
}

impl Default for Projection {
    fn default() -> Self {
        Self {
            kind: ProjectionType::Perspective(Deg(60.0)),
            planes: ClippingPlanes::new(0.0001, 1000.0),
        }
    }
}

impl Projection {
    pub fn new(kind: ProjectionType, planes: ClippingPlanes) -> Self
    {
        Self {
            kind,
            planes,
        }
    }

    pub fn kind(&self) -> &ProjectionType {
        &self.kind
    }

    pub fn planes(&self) -> &ClippingPlanes {
        &self.planes
    }
}
