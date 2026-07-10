use std::time::Instant;

pub struct FrameTimer {
    last_frame: Instant,
    frame_count: u64,
    fps: f64,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            frame_count: 0,
            fps: 0.0,
        }
    }

    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame).as_secs_f64();
        self.frame_count += 1;

        if elapsed >= 1.0 {
            self.fps = self.frame_count as f64 / elapsed;
            self.frame_count = 0;
            self.last_frame = now;
        }

        elapsed
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }

    pub fn latency(&self) -> f64 {
        self.last_frame.elapsed().as_secs_f64() * 1000.0
    }
}
