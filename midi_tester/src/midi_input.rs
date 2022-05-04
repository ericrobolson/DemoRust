extern crate midir;
use crate::midi_message::{MidiMessage, MidiMessageKind};
use midir::{Ignore, MidiInput, MidiInputConnection};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc;

const CLOCKS_PER_QUARTER_NOTE: usize = 24;
const TIMESTAMP_BUFFER_SIZE: usize = CLOCKS_PER_QUARTER_NOTE;

#[derive(Clone, Copy)]
struct MicrosecondTimestamp(u64);

#[allow(dead_code)]
pub struct InputSink {
    /// Need to keep this alive for the entirety of the sink, otherwise things get dropped.
    connection: MidiInputConnection<()>,
    msgs: Vec<MidiMessage>,
    receiver: mpsc::Receiver<MidiMessage>,
    last_timestamps: Vec<MicrosecondTimestamp>,
}
impl InputSink {
    /// Creates a new input sink.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (sender, receiver) = mpsc::channel();

        let mut midi_in = MidiInput::new("midir reading input")?;
        midi_in.ignore(Ignore::None);

        // Get an input port (read from console if multiple are available)
        let in_ports = midi_in.ports();
        let in_port = match in_ports.len() {
            0 => return Err("no input port found".into()),
            1 => {
                println!(
                    "Choosing the only available input port: {}",
                    midi_in.port_name(&in_ports[0]).unwrap()
                );
                &in_ports[0]
            }
            _ => {
                println!("\nAvailable input ports:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_in.port_name(p).unwrap());
                }
                print!("Please select input port: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                in_ports
                    .get(input.trim().parse::<usize>()?)
                    .ok_or("invalid input port selected")?
            }
        };

        println!("\nOpening connection");

        // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
        let connection = midi_in.connect(
            in_port,
            "midir-read-input",
            move |stamp, message, _| {
                if let Some(msg) = MidiMessage::parse_midi(stamp, message) {
                    match sender.send(msg) {
                        Ok(_) => {}
                        Err(e) => println!("{:?}", e),
                    }
                }
            },
            (),
        )?;

        Ok(Self {
            last_timestamps: vec![MicrosecondTimestamp(0); TIMESTAMP_BUFFER_SIZE],
            msgs: vec![],
            connection,
            receiver,
        })
    }

    /// Polls for midi messages.
    pub fn poll(&mut self) -> Option<MidiMessage> {
        for msg in self.receiver.try_iter() {
            // Save timing messages
            if msg.kind == MidiMessageKind::TimingClock {
                if self.last_timestamps.len() >= TIMESTAMP_BUFFER_SIZE {
                    self.last_timestamps.remove(0);
                }

                self.last_timestamps.push(MicrosecondTimestamp(msg.stamp));
            }

            self.msgs.push(msg);
        }

        if self.msgs.is_empty() {
            None
        } else {
            Some(self.msgs.remove(0))
        }
    }
}
