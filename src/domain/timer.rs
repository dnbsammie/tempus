use std::time::{Duration, Instant};

pub struct Timer {
    pub duration: Duration,
    pub start: Option<Instant>,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn remaining(&self) -> Duration {
        if let Some(start) = self.start {
            self.duration.saturating_sub(start.elapsed())
        } else {
            self.duration
        }
    }
}
