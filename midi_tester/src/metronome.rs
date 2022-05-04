use std::time::{Duration, Instant};

//https://en.wikipedia.org/wiki/MIDI_beat_clock
// Clock events are sent at a rate of 24 pulses per quarter note
const CLOCKS_PER_QUARTER_NOTE: u8 = 24;

/// A metronome object.
/// Used for calculating quarter note durations.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Metronome {
    clock_counter: u8,
    last_note: Instant,
    quarter_note_duration: Option<Duration>,
}

impl Metronome {
    /// Creates a new metronome.
    pub fn new() -> Self {
        Self {
            clock_counter: 0,
            last_note: Instant::now(),
            quarter_note_duration: None,
        }
    }

    /// Ticks the clock from a midi event.
    /// Returns the most updated quarter note duration.
    pub fn tick_clock(&mut self) -> Option<QuarterNoteDuration> {
        self.clock_counter += 1;

        if self.clock_counter == CLOCKS_PER_QUARTER_NOTE {
            self.clock_counter = 0;
            let now = Instant::now();
            let quarter_note_duration = now - self.last_note;
            self.last_note = now;
            Some(QuarterNoteDuration(quarter_note_duration))
        } else {
            None
        }
    }
}

pub struct QuarterNoteDuration(Duration);
impl QuarterNoteDuration {
    pub fn to_duration(self) -> Duration {
        self.0
    }
}
