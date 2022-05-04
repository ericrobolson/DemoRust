type M = MidiMessageKind;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MidiMessage {
    pub kind: MidiMessageKind,
    pub stamp: u64,
}
impl MidiMessage {
    pub fn parse_midi(stamp: u64, msg: &[u8]) -> Option<MidiMessage> {
        let kind = match msg {
            // Notes
            [128, note_number, note_velocity] => Some(M::Note {
                channel: 0,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [129, note_number, note_velocity] => Some(M::Note {
                channel: 1,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [130, note_number, note_velocity] => Some(M::Note {
                channel: 2,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [131, note_number, note_velocity] => Some(M::Note {
                channel: 3,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [132, note_number, note_velocity] => Some(M::Note {
                channel: 4,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [133, note_number, note_velocity] => Some(M::Note {
                channel: 5,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [134, note_number, note_velocity] => Some(M::Note {
                channel: 6,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [135, note_number, note_velocity] => Some(M::Note {
                channel: 7,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [136, note_number, note_velocity] => Some(M::Note {
                channel: 8,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [137, note_number, note_velocity] => Some(M::Note {
                channel: 9,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [138, note_number, note_velocity] => Some(M::Note {
                channel: 10,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [139, note_number, note_velocity] => Some(M::Note {
                channel: 11,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [140, note_number, note_velocity] => Some(M::Note {
                channel: 12,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [141, note_number, note_velocity] => Some(M::Note {
                channel: 13,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [142, note_number, note_velocity] => Some(M::Note {
                channel: 14,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [143, note_number, note_velocity] => Some(M::Note {
                channel: 15,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::Off,
            }),
            [144, note_number, note_velocity] => Some(M::Note {
                channel: 0,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [145, note_number, note_velocity] => Some(M::Note {
                channel: 1,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [146, note_number, note_velocity] => Some(M::Note {
                channel: 2,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [147, note_number, note_velocity] => Some(M::Note {
                channel: 3,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [148, note_number, note_velocity] => Some(M::Note {
                channel: 4,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [149, note_number, note_velocity] => Some(M::Note {
                channel: 5,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [150, note_number, note_velocity] => Some(M::Note {
                channel: 6,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [151, note_number, note_velocity] => Some(M::Note {
                channel: 7,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [152, note_number, note_velocity] => Some(M::Note {
                channel: 8,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [153, note_number, note_velocity] => Some(M::Note {
                channel: 9,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [154, note_number, note_velocity] => Some(M::Note {
                channel: 10,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [155, note_number, note_velocity] => Some(M::Note {
                channel: 11,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [156, note_number, note_velocity] => Some(M::Note {
                channel: 12,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [157, note_number, note_velocity] => Some(M::Note {
                channel: 13,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [158, note_number, note_velocity] => Some(M::Note {
                channel: 14,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),
            [159, note_number, note_velocity] => Some(M::Note {
                channel: 15,
                note_number: *note_number,
                velocity: *note_velocity,
                state: State::On,
            }),

            //
            [248] => Some(M::TimingClock),
            [250] => Some(M::Start),
            [251] => Some(M::Continue),
            [252] => Some(M::Stop),
            _ => {
                println!("Unhandled midi: {}: {:?} (len = {})\n\thttps://www.midi.org/specifications-old/item/table-2-expanded-messages-list-status-bytes", stamp, msg, msg.len());
                None
            }
        };

        match kind {
            Some(kind) => Some(MidiMessage { stamp, kind }),
            None => None,
        }
    }
}

/// Midi messages
/// https://www.midi.org/specifications-old/item/table-2-expanded-messages-list-status-bytes
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MidiMessageKind {
    Note {
        channel: u8,
        note_number: u8,
        velocity: u8,
        state: State,
    },
    TimingClock,
    Start,
    Continue,
    Stop,
}
impl MidiMessageKind {
    pub fn raw_midi<'a>(&self) -> Vec<u8> {
        match self {
            MidiMessageKind::Note {
                channel,
                note_number,
                velocity,
                state,
            } => vec![144 + *channel, 60, 127],
            MidiMessageKind::TimingClock => todo!(),
            MidiMessageKind::Start => todo!(),
            MidiMessageKind::Continue => todo!(),
            MidiMessageKind::Stop => todo!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    On,
    Off,
}
