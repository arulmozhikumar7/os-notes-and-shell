use std::ffi::CString;
use std::process;
use std::ptr;
use libc::{fork, execvp, waitpid, c_char, signal, SIGINT, SIG_DFL};

pub fn execute_commands(input: &str) {
    let commands: Vec<&str> = input.split("&&").map(|s| s.trim()).collect();
    let mut success = true;

    for cmd in commands {
        if !success {
            break;
        }

        let background = cmd.ends_with('&');
        let actual = if background {
            cmd.trim_end_matches('&').trim()
        } else {
            cmd
        };

        let parts: Vec<CString> = actual
            .split_whitespace()
            .map(|s| CString::new(s).unwrap())
            .collect();

        if parts.is_empty() {
            continue;
        }

        let mut c_args: Vec<*const c_char> = parts.iter().map(|s| s.as_ptr()).collect();
        c_args.push(ptr::null());

        unsafe {
            let pid = fork();
            if pid < 0 {
                eprintln!("Fork failed");
                break;
            }

            if pid == 0 {
                signal(SIGINT, SIG_DFL);
                execvp(c_args[0], c_args.as_ptr());
                eprintln!("Unknown command: {}", actual);
                process::exit(1);
            } else {
                if background {
                    println!("[Running in background] pid={}", pid);
                } else {
                    let mut status = 0;
                    waitpid(pid, &mut status, 0);
                    success = status == 0;
                }
            }
        }
    }
}
