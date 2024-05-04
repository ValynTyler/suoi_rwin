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
}

impl Mouse {
    pub fn update_position(&mut self, pos: Vector2) {
        self.position = pos;
    }
}
