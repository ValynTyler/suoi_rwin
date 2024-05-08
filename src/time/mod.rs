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
    /// Updates `Time` object
    pub fn poll(&mut self, context: &Context) {
        self.seconds_last_frame = self.seconds;
        self.seconds = context.glfw().get_time() as f32;
    }

    /// Returns `delta_time` since last `Time::poll()`
    pub fn delta(&self) -> f32 {
        self.seconds - self.seconds_last_frame
    }
    
    /// Returns the current framerate
    pub fn framerate(&self) -> f32 {
        1.0 / self.delta()
    }
    
    /// Returns the time since launch in seconds
    pub fn seconds(&self) -> f32 {
        self.seconds
    }

    /// Returns the time since launch in milliseconds
    pub fn millis(&self) -> u32 {
        (self.seconds * 1000.0) as u32
    }
}
