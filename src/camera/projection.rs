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

pub struct Projection {
    kind: ProjectionType,
    planes: ClippingPlanes,
    fov: Angle,
}

impl Projection {
    pub fn new(kind: ProjectionType, planes: ClippingPlanes, fov: Angle) -> Self {
        Self {
            kind,
            planes,
            fov,
        }
    }
    
    pub fn kind(&self) -> &ProjectionType {
        &self.kind
    }
    
    pub fn planes(&self) -> &ClippingPlanes {
        &self.planes
    }
    
    pub fn fov(&self) -> Angle {
        self.fov
    }
}
