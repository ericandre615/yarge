use std::time::{Instant, Duration};

pub struct Timer {
    start: Instant,
    last_frame_time: Instant,
    frame_count: usize,
    delta_time: f32
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: Instant::now(),
            last_frame_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let last_frame_time = now - self.last_frame_time;
        //let delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        self.delta_time = last_frame_time.as_millis() as f32;
        self.frame_count += 1;
    }

    pub fn last_frame_time(&self) -> Instant {
        self.last_frame_time
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
}
