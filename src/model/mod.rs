pub mod vertex;
pub use vertex::*;

pub mod texture;
pub use texture::*;

pub mod mesh;
pub use mesh::*;

pub mod model;
pub use model::*;

use gl::types::*;

pub const SIZE_OF_FLOAT: GLsizei = std::mem::size_of::<GLfloat>() as GLsizei;
