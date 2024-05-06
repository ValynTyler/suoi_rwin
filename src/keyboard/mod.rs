use crate::Context;

pub enum Key {
    W,
    A,
    S,
    D,
    Esc,
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
    pub fn get_key(key: Key, ctx: &Context) -> KeyState {
        match key {
            Key::W => KeyState::from(ctx.window().get_key(glfw::Key::W)),
            Key::A => KeyState::from(ctx.window().get_key(glfw::Key::A)),
            Key::S => KeyState::from(ctx.window().get_key(glfw::Key::S)),
            Key::D => KeyState::from(ctx.window().get_key(glfw::Key::D)),
            Key::Esc => KeyState::from(ctx.window().get_key(glfw::Key::Escape)),
        }
    }

    /**
    # `Keyboard::input_axis`
    ---
    Returns a `f32` float value contained in the range `[-1.0, 1.0]`,
    where -1.0 means that only the `neg_key` is pressed and +1.0 means
    that only the `pos_key` is (0 could mean that they are both pressed
    or that neither are).
    */
    pub fn input_axis(ctx: &Context, neg_key: Key, pos_key: Key) -> f32 {
        (match Keyboard::get_key(neg_key, ctx) {
            KeyState::Pressed => -1.0,
            KeyState::Released => 0.0,
        })

        +

        (match Keyboard::get_key(pos_key, ctx) {
            KeyState::Pressed => 1.0,
            KeyState::Released => 0.0,
        })
    }
}
