use core::ops::{AddAssign, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerResult {
    Ticked,
    NotTicked,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameTimer {
    accumulated_time: AccumulatedSeconds,
    tick_hz: TickHz,
}
impl GameTimer {
    /// Creates a new game timer that triggers at the given rate
    pub fn new(tick_hz: f32) -> Self {
        Self {
            accumulated_time: AccumulatedSeconds(0.0),
            tick_hz: TickHz(tick_hz),
        }
    }

    /// Updates the timer.
    pub fn update(&mut self, delta_t_seconds: f32) {
        self.accumulated_time += ElapsedSeconds(delta_t_seconds);
    }

    /// Ticks the timer.
    pub fn tick(&mut self) -> TimerResult {
        let mut result = TimerResult::NotTicked;
        if should_tick(self.accumulated_time, self.tick_hz) {
            self.accumulated_time -= self.tick_hz;
            result = TimerResult::Ticked
        }

        result
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct AccumulatedSeconds(f32);

#[derive(Clone, Copy, PartialEq, Debug)]
struct ElapsedSeconds(f32);

#[derive(Clone, Copy, PartialEq, Debug)]
struct TickHz(f32);

fn should_tick(acc: AccumulatedSeconds, delta_t: TickHz) -> bool {
    acc.0 >= delta_t.0
}

impl AddAssign<ElapsedSeconds> for AccumulatedSeconds {
    fn add_assign(&mut self, rhs: ElapsedSeconds) {
        self.0 += rhs.0;
    }
}
impl SubAssign<ElapsedSeconds> for AccumulatedSeconds {
    fn sub_assign(&mut self, rhs: ElapsedSeconds) {
        self.0 -= rhs.0;
    }
}
impl SubAssign<TickHz> for AccumulatedSeconds {
    fn sub_assign(&mut self, rhs: TickHz) {
        self.0 -= rhs.0
    }
}
