use std::time::{Duration, Instant};

/// A simple timer struct.
/// It works by calling `tick` repeatedly.
#[derive(Clone, Copy)]
pub struct Timer {
    last_ticked: Instant,
    accumulated: Duration,
    tick_rate: Duration,
}

impl Timer {
    /// Creates a new timer.
    pub fn new(hz: u32) -> Self {
        let tick_rate = 1.0 / (hz as f32);

        Self {
            last_ticked: Instant::now(),
            accumulated: Duration::from_micros(0),
            tick_rate: Duration::from_secs_f32(tick_rate),
        }
    }

    /// Ticks the timer.
    pub fn tick(&mut self) -> TickResult {
        let now = Instant::now();
        let delta = now - self.last_ticked;
        self.accumulated += delta;
        let mut triggered = false;

        if self.accumulated >= self.tick_rate {
            self.accumulated -= self.tick_rate;
            triggered = true;
        }

        TickResult { triggered }
    }
}

/// The result of a tick.
pub struct TickResult {
    pub triggered: bool,
}
