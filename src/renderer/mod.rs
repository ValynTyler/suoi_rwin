use nerd::color::Color;

pub struct Renderer {}

impl Renderer {
    pub unsafe fn init() {
        // Wireframe
        let wireframe_on = false;
        if wireframe_on {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }

        // Backface culling
        let backface_culling = true;
        if backface_culling {
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CCW);
        }

        // Z-Buffer
        gl::Enable(gl::DEPTH_TEST);
    }

    pub unsafe fn clear_screen(col: Color) {
        gl::ClearColor(
            col.r as f32 / 255.,
            col.g as f32 / 255.,
            col.b as f32 / 255.,
            1.0,
        );
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}
