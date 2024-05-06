use nerd::vector::Vector3;

#[allow(unused)]
pub struct CoordSpace {
    pub(crate) origin: Vector3,
    pub(crate) up: Vector3,
    pub(crate) right: Vector3,
    pub(crate) forward: Vector3,
}

impl Default for CoordSpace {
    fn default() -> Self {
        Self {
            origin: Vector3::ZERO,
            up: Vector3::UP,
            right: Vector3::RIGHT,
            forward: Vector3::FORWARD,
        }
    }
}

impl CoordSpace {
    pub fn new(origin: Vector3, up: Vector3, right: Vector3, forward: Vector3) -> Self {
        Self {
            origin,
            up,
            right,
            forward
        }
    }

    pub fn from_fwd_and_world_up(fwd: Vector3, world_up: Vector3) -> Self {
        Self::new(Vector3::ZERO, world_up, Vector3::cross(fwd, world_up), fwd)
    }
}
