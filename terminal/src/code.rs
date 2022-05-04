use std::io::Read;
use std::time::Duration;

fn display_processes() {
    use sysinfo::{ProcessExt, System, SystemExt};

    let s = System::new_all();
    for (pid, process) in s.processes() {
        println!(
            "pid: {} name: {}, exe: {}",
            pid,
            process.name(),
            process.exe().display()
        );
    }
}

/// Prints and clears the terminal
fn print_and_clear() {
    use std::io::{self, Write};

    io::stdout().write_all(b"hello world").unwrap();

    print!("\x1B[2J\x1B[1;1H");
}
