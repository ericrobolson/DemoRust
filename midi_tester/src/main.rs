mod example_midi_out;
mod metronome;
mod midi_input;
mod midi_message;
mod midi_output;
mod sequencer;

use midi_message::MidiMessageKind;
use std::time::Duration;

fn main() {
    let mut sink = match midi_input::InputSink::new() {
        Ok(s) => s,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let mut output = match midi_output::OutputSink::new() {
        Ok(output) => output,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let mut metronome = metronome::Metronome::new();
    let mut sequencer = sequencer::Sequencer::new();

    loop {
        if let Some(msg) = sink.poll() {
            match msg.kind {
                MidiMessageKind::Continue => {
                    sequencer.handle_event(sequencer::Event::ContinuePlayback)
                }
                MidiMessageKind::Start => {
                    sequencer.handle_event(sequencer::Event::StartPlayback);
                }
                MidiMessageKind::Stop => sequencer.handle_event(sequencer::Event::StopPlayback),
                MidiMessageKind::TimingClock => match metronome.tick_clock() {
                    Some(duration) => {
                        let duration = duration.to_duration();
                        println!(
                            "BPM: {:?}",
                            Duration::from_secs(60).as_secs_f64() / duration.as_secs_f64()
                        );

                        sequencer.handle_event(sequencer::Event::SetQuarterNoteDuration(duration));
                    }
                    None => {}
                },
                kind => println!("{:?}", kind),
            }
        }

        for msg in sequencer.tick() {
            output.send(msg).unwrap();
        }
    }
}
