use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use termios::*;

static mut ORIGINAL_TERMIOS: Option<Termios> = None;

pub fn enable_raw_mode() -> Termios {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd).unwrap();
    let original = termios.clone();

    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin_fd, TCSANOW, &termios).unwrap();

    unsafe {
        ORIGINAL_TERMIOS = Some(original.clone());
    }

    original
}

pub fn disable_raw_mode(original: Termios) {
    let stdin_fd = io::stdin().as_raw_fd();
    tcsetattr(stdin_fd, TCSANOW, &original).unwrap();
}

extern "C" fn sigint_handler(_sig: libc::c_int) {
    print!("\n> ");
    io::stdout().flush().unwrap();
}

pub fn handle_sigint() -> extern "C" fn(libc::c_int) {
    sigint_handler
}
