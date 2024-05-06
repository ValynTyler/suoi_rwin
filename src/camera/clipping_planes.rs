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