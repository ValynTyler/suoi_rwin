use crate::Context;

pub struct Time {
    seconds: f32,
    seconds_last_frame: f32,
}

impl Default for Time {
    fn default() -> Self {
        Self { seconds: Default::default(), seconds_last_frame: Default::default() }
    }
}

impl Time {
    pub fn poll(&mut self, context: &Context) {
        self.seconds_last_frame = self.seconds;
        self.seconds = context.glfw().get_time() as f32;
    }

    pub fn delta(&self) -> f32 {
        self.seconds - self.seconds_last_frame
    }
    
    pub fn seconds(&self) -> f32 {
        self.seconds
    }

    pub fn millis(&self) -> u32 {
        (self.seconds * 1000.0) as u32
    }
}
