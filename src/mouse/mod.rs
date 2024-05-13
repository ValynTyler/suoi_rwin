use suoi_types::Vector2;

use crate::{Context, Screen};

pub struct MouseButton {
    is_pressed: bool,
    was_pressed_lf: bool,
}

impl Default for MouseButton {
    fn default() -> Self {
        Self {
            is_pressed: Default::default(),
            was_pressed_lf: Default::default(),
        }
    }
}

impl MouseButton {
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn just_pressed(&self) -> bool {
        self.is_pressed == true && self.was_pressed_lf == false
    }
}

pub struct Mouse {
    left_button: MouseButton,
    right_button: MouseButton,
    
    position: Vector2,
    lf_position: Vector2,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            left_button: Default::default(),
            right_button: Default::default(),

            position: Default::default(),
            lf_position: Default::default(),
        }
    }
}

impl Mouse {
    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn delta(&self) -> Vector2 {
        self.position - self.lf_position
    }

    pub fn poll(&mut self, ctx: &Context) {
        // delta
        self.lf_position = self.position;
        self.left_button.was_pressed_lf = self.left_button.is_pressed;
        self.right_button.was_pressed_lf = self.right_button.is_pressed;

        // Mouse 1
        self.left_button.is_pressed =
            match ctx.window().get_mouse_button(glfw::MouseButton::Button1) {
                glfw::Action::Press => true,
                glfw::Action::Repeat => true,
                glfw::Action::Release => false,
            };

        // Mouse 1
        self.right_button.is_pressed =
            match ctx.window().get_mouse_button(glfw::MouseButton::Button2) {
                glfw::Action::Press => true,
                glfw::Action::Repeat => true,
                glfw::Action::Release => false,
            };
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
            y: self.position().y / screen.height() as f32 * -2.0 + 1.0,
        }
    }
}
