extern crate midir;
use midir::{Ignore, MidiInput, MidiOutputConnection};
use midir::{MidiOutput, MidiOutputPort};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::midi_message::{MidiMessage, MidiMessageKind, State};

pub struct OutputSink {
    connection: MidiOutputConnection,
}
impl OutputSink {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let midi_out = MidiOutput::new("My Test Output")?;

        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_port: &MidiOutputPort = match out_ports.len() {
            0 => return Err("no output port found".into()),
            1 => {
                println!(
                    "Choosing the only available output port: {}",
                    midi_out.port_name(&out_ports[0]).unwrap()
                );
                &out_ports[0]
            }
            _ => {
                println!("\nAvailable output ports:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                out_ports
                    .get(input.trim().parse::<usize>()?)
                    .ok_or("invalid output port selected")?
            }
        };

        println!("\nOpening connection");
        let mut connection = midi_out.connect(out_port, "midir-test")?;
        println!("Connection open. Listen!");

        Ok(Self { connection })
    }

    pub fn send(&mut self, note: MidiMessageKind) -> Result<(), Box<dyn Error>> {
        match self.connection.send(&note.raw_midi()) {
            Ok(()) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
