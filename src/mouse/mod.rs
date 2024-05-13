use suoi_types::Vector2;

use crate::{Context, Screen};

#[allow(unused)]
pub struct MouseButton {
    is_pressed: bool,
}

impl Default for MouseButton {
    fn default() -> Self {
        Self {
            is_pressed: Default::default(),
        }
    }
}

impl MouseButton {
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

#[allow(unused)]
pub struct Mouse {
    left_button: MouseButton,
    right_button: MouseButton,

    position: Vector2,
    last_pos: Vector2,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            left_button: Default::default(),
            right_button: Default::default(),
            position: Default::default(),
            last_pos: Default::default(),
        }
    }
}

impl Mouse {
    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn delta(&self) -> Vector2 {
        self.position - self.last_pos
    }

    pub fn poll(&mut self, ctx: &Context) {
        // Mouse 1
        self.left_button.is_pressed =  match ctx.window().get_mouse_button(glfw::MouseButton::Button1) {
            glfw::Action::Press => true,
            glfw::Action::Repeat => true,
            glfw::Action::Release => false,
        };

        // Mouse 1
        self.right_button.is_pressed =  match ctx.window().get_mouse_button(glfw::MouseButton::Button2) {
            glfw::Action::Press => true,
            glfw::Action::Repeat => true,
            glfw::Action::Release => false,
        };
        
        // Mouse Delta
        self.last_pos = self.position;
    }

    pub fn update_position(&mut self, pos: Vector2) {
        self.position = pos;
    }
    
    pub fn left_button(&self) -> &MouseButton {
        &self.left_button
    }
    
    pub fn right_button(&self) -> &MouseButton {
        &self.right_button
    }

    pub fn ndc(&self, screen: &Screen) -> Vector2 {
        Vector2 {
            x: self.position().x / screen.width() as f32 * 2.0 - 1.0,
            y: self.position().y / screen.height() as f32 * -2.0 + 1.0
        }
    }
}
