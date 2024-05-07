use nerd::{matrix::Matrix4, vector::Vector3};

use crate::{ClippingPlanes, CoordSpace, ProjectionType, Screen};

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

    pub fn world_up(&self) -> Vector3 {
        self.global_space.up
    }

    pub fn forward(&self) -> Vector3 {
        self.local_space.forward
    }

    pub fn right(&self) -> Vector3 {
        self.local_space.right
    }

    pub fn up(&self) -> Vector3 {
        self.local_space.up
    }

    pub fn position_mut(&mut self) -> &mut Vector3 {
        &mut self.local_space.origin
    }

    pub fn forward_mut(&mut self) -> &mut Vector3 {
        &mut self.local_space.forward
    }

    pub fn right_mut(&mut self) -> &mut Vector3 {
        &mut self.local_space.right
    }

    pub fn up_mut(&mut self) -> &mut Vector3 {
        &mut self.local_space.up
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
