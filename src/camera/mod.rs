use nerd::{matrix::Matrix4, vector::Vector3};

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

#[allow(unused)]
pub struct CoordSpace {
    origin: Vector3,
    up: Vector3,
    right: Vector3,
    forward: Vector3,
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

#[allow(unused)]
pub struct Camera {
    projection_type: ProjectionType,
    clipping_planes: ClippingPlanes,
    fov: f32,

    local_space: CoordSpace,
    global_space: CoordSpace,
}

impl Camera {
    pub fn new(projection: ProjectionType, clipping: ClippingPlanes, fov: f32) -> Self {
        Self {
            projection_type: projection,
            clipping_planes: clipping,
            fov,
            local_space: CoordSpace::from_fwd_and_world_up(Vector3::new(0.0, 0.0, -1.0), Vector3::UP),
            global_space: CoordSpace::default(),
        }
    }

    pub fn view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(
            self.local_space.origin,
            self.local_space.origin + self.local_space.forward,
            self.local_space.up,
        )
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
