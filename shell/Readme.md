# Rust Mini Shell

A minimalist UNIX-like shell written in Rust that supports:

* Raw terminal input with real-time editing (arrow key navigation, backspace, cursor movement)
* Command history
* Background execution with `&`
* Command chaining with `&&`

---

## 🧠 Features

* **Raw Mode Input**: Processes input character-by-character instead of line-by-line
* **Arrow Key Support**: Navigate left/right within a command or up/down through command history
* **Process Execution**: Uses `fork` and `execvp` to spawn processes
* **Signal Handling**: Handles Ctrl+C gracefully

---

## 📁 Project Structure

```bash
.
├── main.rs              # Entry point
├── terminal.rs          # Raw mode enabling/disabling, signal handlers
├── input.rs             # Handles line editing and input reading
├── executor.rs          # Parses and runs commands
└── README.md            # Project overview
```

---

## ⚙️ How It Works

1. **Raw Mode** is enabled using the `termios` crate to bypass canonical input buffering.
2. On each keystroke:

   * Character is inserted into the buffer
   * Cursor position is updated
   * Line is redrawn using ANSI escape sequences
3. When Enter is pressed, the buffer is parsed and split by `&&` to support command chaining.
4. If any command ends with `&`, it is run in the background.
5. Ctrl+C simply redraws the prompt without killing the shell.

---

## 🚀 Run

```bash
cargo run
```

> Requires a UNIX-like environment (Linux/macOS).
---

## 👤 Author

Built by Arulmozhikumar while learning OS and systems programming using Rust.
