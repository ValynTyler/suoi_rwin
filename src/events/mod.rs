use nerd::vector::Vector2;

use crate::{Context, Mouse, Screen};

pub enum Event {
    ResizedWindow(u32, u32),
    MovedCursor(Vector2),
}

#[allow(unused)]
pub struct EventHandler {
    events: Vec<Event>,
}

impl EventHandler {
    pub fn poll_events(&mut self, ctx: &mut Context, screen: &mut Screen, mouse: &mut Mouse) {
        // Poll and handle events
        ctx.glfw().poll_events();
        for (_, event) in glfw::flush_messages(&ctx.events()) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    let (width, height) = (width as u32, height as u32);

                    screen.resize(width, height);

                    self.events.push(Event::ResizedWindow(width, height))
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let pos = Vector2::new(xpos as f32, ypos as f32);

                    mouse.update_position(pos);

                    self.events.push(Event::MovedCursor(pos))
                }
                _ => {}
            }
        }
    }
}
