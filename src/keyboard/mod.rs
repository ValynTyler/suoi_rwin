use crate::Context;

pub enum Key {
    W,
    A,
    S,
    D,
}

pub enum KeyState {
    Pressed,
    Released,
}

impl KeyState {
    pub fn is_pressed(&self) -> bool {
        match self {
            KeyState::Pressed => true,
            KeyState::Released => false,
        }
    }

    pub fn is_released(&self) -> bool {
        match self {
            KeyState::Pressed => false,
            KeyState::Released => true,
        }
    }
}

impl From<glfw::Action> for KeyState {
    fn from(value: glfw::Action) -> Self {
        match value {
            glfw::Action::Repeat => KeyState::Pressed,
            glfw::Action::Press => KeyState::Pressed,
            glfw::Action::Release => KeyState::Released,
        }
    }
}

pub struct Keyboard;

impl Keyboard {
    pub fn get_key(&self, key: Key, ctx: Context) -> KeyState {
        match key {
            Key::W => KeyState::from(ctx.window().get_key(glfw::Key::W)),
            Key::A => KeyState::from(ctx.window().get_key(glfw::Key::A)),
            Key::S => KeyState::from(ctx.window().get_key(glfw::Key::S)),
            Key::D => KeyState::from(ctx.window().get_key(glfw::Key::D)),
        }
    }
}
