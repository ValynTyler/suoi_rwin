pub enum ProjectionType {
    Ortho,
    Perspective,
}

#[allow(unused)]
pub struct Camera {
    projection_type: ProjectionType,
}