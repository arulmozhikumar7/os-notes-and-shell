
mod terminal;
mod input;
mod executor;

use libc::{signal, SIGINT};
use terminal::{enable_raw_mode, disable_raw_mode, handle_sigint};
use input::read_command;
use executor::execute_commands;

fn main() {
    unsafe {
        signal(SIGINT, handle_sigint as usize);
    }

    let original_termios = enable_raw_mode();
    let mut history = Vec::new();

    loop {
        let input = read_command(&mut history);

        let trimmed = input.trim();
        if trimmed == "exit" {
            println!("Exiting shell...");
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        history.push(trimmed.to_string());
        execute_commands(trimmed);
    }

    disable_raw_mode(original_termios);
}
