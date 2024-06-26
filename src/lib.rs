pub mod screen;
pub use screen::*;

pub mod context;
pub use context::*;

pub mod events;
pub use events::*;

pub mod mouse;
pub use mouse::*;

pub mod keyboard;
pub use keyboard::*;

pub mod time;
pub use time::*;

pub mod renderer;
pub use renderer::*;

pub mod camera;
pub use camera::*;

pub mod shader;
pub use shader::*;

pub mod model;
pub use model::*;

pub mod line;
pub use line::*;

pub trait GraphicsObject {
    unsafe fn with<T>(&self, operation: T)
    where
        T: FnMut();
}
