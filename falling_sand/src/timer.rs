use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Timer {
    hz: u32,
    last_tick: Instant,
    accumulated: Duration,
    tick_duration: Duration,
}
impl Timer {
    /// Creates a new timer.
    pub fn new(hz: u32) -> Self {
        Self {
            hz,
            last_tick: Instant::now(),
            accumulated: Duration::default(),
            tick_duration: Duration::from_secs_f32(1.0 / hz as f32),
        }
    }

    /// Returns whether the timer has ticked or not.
    pub fn ticked(&mut self) -> bool {
        let now = Instant::now();
        let delta_t = now - self.last_tick;
        self.last_tick = now;

        self.accumulated += delta_t;

        if self.accumulated >= self.tick_duration {
            self.accumulated -= self.tick_duration;
            true
        } else {
            false
        }
    }
}
