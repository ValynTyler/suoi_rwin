use nerd::vector::Vector2;

#[allow(unused)]
struct MouseButton {
    is_clicked: bool,
}

#[allow(unused)]
pub struct Mouse {
    left_button: MouseButton,
    right_button: MouseButton,
    
    position: Vector2,
    last_pos: Vector2,
}

impl Mouse {
    pub fn delta(&self) -> Vector2 {
        self.position - self.last_pos
    }

    pub fn update_position(&mut self, pos: Vector2) {
        self.last_pos = self.position;
        self.position = pos;
    }
}
