use std::io::{self, Read, Write};

fn redraw(buffer: &str, cursor: usize) {
    print!("\r> {} ", buffer);
    print!("\r\x1b[{}C", cursor + 2);
    io::stdout().flush().unwrap();
}

pub fn read_command(history: &mut Vec<String>) -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let mut cursor: usize = 0;
    let mut history_index: Option<usize> = None;

    loop {
        let mut byte = [0; 1];
        io::stdin().read_exact(&mut byte).unwrap();

        match byte[0] {
            b'\n' => {
                println!();
                break;
            }
            b'\x1b' => {
                let mut seq = [0; 2];
                io::stdin().read_exact(&mut seq).unwrap();
                match seq {
                    [b'[', b'A'] => {
                        if let Some(i) = history_index {
                            if i > 0 {
                                history_index = Some(i - 1);
                            }
                        } else if !history.is_empty() {
                            history_index = Some(history.len() - 1);
                        }

                        if let Some(i) = history_index {
                            buffer = history[i].clone();
                            cursor = buffer.len();
                            redraw(&buffer, cursor);
                        }
                    }
                    [b'[', b'B'] => {
                        if let Some(i) = history_index {
                            if i + 1 < history.len() {
                                history_index = Some(i + 1);
                                buffer = history[i + 1].clone();
                            } else {
                                history_index = None;
                                buffer.clear();
                            }
                            cursor = buffer.len();
                            redraw(&buffer, cursor);
                        }
                    }
                    [b'[', b'D'] => {
                        if cursor > 0 {
                            cursor -= 1;
                            print!("\x1b[1D");
                            io::stdout().flush().unwrap();
                        }
                    }
                    [b'[', b'C'] => {
                        if cursor < buffer.len() {
                            cursor += 1;
                            print!("\x1b[1C");
                            io::stdout().flush().unwrap();
                        }
                    }
                    _ => {}
                }
            }
            127 => {
                if cursor > 0 {
                    cursor -= 1;
                    buffer.remove(cursor);
                    redraw(&buffer, cursor);
                }
            }
            byte => {
                buffer.insert(cursor, byte as char);
                cursor += 1;
                redraw(&buffer, cursor);
            }
        }
    }

    buffer
}