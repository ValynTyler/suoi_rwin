use glfw::{Glfw, GlfwReceiver, PWindow, WindowEvent};
use glfw::Context as GLFWContext;

type Events = GlfwReceiver<(f64, WindowEvent)>;

use crate::Screen;

#[allow(unused)]
pub struct Context {
    glfw: Glfw,
    window: PWindow,
    events: Events,
}

impl Context {
    pub fn init(screen: &Screen) -> Self {
        // GLFW initialization
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(
                screen.width(),
                screen.height(),
                "resource-manager-test",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        // load GL functions for the current window
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Self {
            glfw,
            window,
            events
        }
    }

    pub fn running(&self) -> bool {
        !self.window.should_close()
    }
    
    pub fn glfw(&self) -> &Glfw {
        &self.glfw
    }
    
    pub fn window(&self) -> &PWindow {
        &self.window
    }
    
    pub fn events(&self) -> &Events {
        &self.events
    }

    pub fn glfw_mut(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    pub fn window_mut(&mut self) -> &mut PWindow {
        &mut self.window
    }
}
