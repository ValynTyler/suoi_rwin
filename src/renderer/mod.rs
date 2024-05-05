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
}
