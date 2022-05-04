use std::time::{Duration, Instant};

use crate::midi_message::MidiMessageKind;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Playing,
    Paused,
}

pub struct Sequencer {
    last_tick: Instant,
    accumulated: Duration,
    quarter_note_duration: Duration,
    status: Status,
}
impl Sequencer {
    /// Creates a new sequencer
    pub fn new() -> Self {
        Self {
            status: Status::Paused,
            last_tick: Instant::now(),
            accumulated: Duration::default(),
            quarter_note_duration: Duration::from_secs_f64(128.0 / 60.0),
        }
    }

    /// Ticks the sequencer.
    pub fn tick(&mut self) -> Vec<MidiMessageKind> {
        let now = Instant::now();
        self.accumulated += now - self.last_tick;
        self.last_tick = now;

        let mut notes = vec![];

        if self.accumulated >= self.quarter_note_duration {
            self.accumulated -= self.quarter_note_duration;

            if self.status == Status::Playing {
                println!("Ticked!");

                for channel in 0..5 {
                    notes.push(MidiMessageKind::Note {
                        channel,
                        note_number: 60,
                        velocity: 127,
                        state: crate::midi_message::State::On,
                    });
                    notes.push(MidiMessageKind::Note {
                        channel,
                        note_number: 60,
                        velocity: 0,
                        state: crate::midi_message::State::On,
                    });
                }
            }
        }

        notes
    }

    /// Handles an event
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::StartPlayback => {
                println!("Start playback");
                self.status = Status::Playing;
                // TODO: restart at beginning
            }
            Event::ContinuePlayback => {
                println!("Continue playback");
                self.status = Status::Playing;
                // TODO: continue from last position
            }
            Event::StopPlayback => {
                println!("Stop playback");
                self.status = Status::Paused;
            }
            Event::SetQuarterNoteDuration(duration) => self.quarter_note_duration = duration,
        }
    }
}

pub enum Event {
    SetQuarterNoteDuration(Duration),
    StartPlayback,
    ContinuePlayback,
    StopPlayback,
}
