use nerd::vector::Vector2;

#[allow(unused)]
struct MouseButton {
    is_clicked: bool,
}

impl Default for MouseButton {
    fn default() -> Self {
        Self {
            is_clicked: Default::default(),
        }
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
            position: Vector2::ZERO,
            last_pos: Vector2::ZERO,
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

    pub fn poll(&mut self) {
        self.last_pos = self.position
    }

    pub fn update_position(&mut self, pos: Vector2) {
        self.position = pos;
    }
}
