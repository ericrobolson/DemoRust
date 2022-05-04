mod code;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::position,
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, ClearType},
    Result,
};

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

const EXIT: Event = Event::Key(KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
});

fn clear_screen(stdout: &mut std::io::Stdout) {
    //        let mut stdout = stdout();

    queue!(stdout, crossterm::terminal::Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();
}

fn print_total_screen() {
    let (w, h) = crossterm::terminal::size().unwrap();
    for h in 0..h {
        let mut s = String::new();
        for w in 0..w {
            s.push('c');
        }
        println!("{}", s);
    }
}

fn print_events() -> Result<()> {
    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);
            println!("size: {:?}", crossterm::terminal::size());

            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == EXIT {
                break;
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            // Timeout expired, no event for 1s
            println!(".\r");
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}
