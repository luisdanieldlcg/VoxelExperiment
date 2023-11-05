use std::time::{Duration, Instant};

pub struct Clock {
    last_time: Instant,
    dt: Duration,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            last_time: Instant::now(),
            dt: Duration::ZERO,
        }
    }
}

impl Clock {
    pub fn tick(&mut self) {
        self.dt = self.last_time.elapsed();
        self.last_time = Instant::now();
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }
}
