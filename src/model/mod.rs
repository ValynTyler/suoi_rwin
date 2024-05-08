pub mod vertex;
pub use vertex::*;

pub mod mesh;
pub use mesh::*;

use gl::types::*;

pub const SIZE_OF_FLOAT: GLsizei = std::mem::size_of::<GLfloat>() as GLsizei;
