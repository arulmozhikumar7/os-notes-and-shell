# Canonical vs Non-Canonical Mode in Terminals

In Unix-like systems, terminal input behavior is managed through terminal modes. The two primary modes that affect how input is handled are **canonical mode** and **non-canonical mode**.

---

## 🔹 Canonical Mode (Cooked Mode)

* This is the **default** terminal mode.
* Input is **line-buffered**:

  * The kernel waits for the **Enter key (\n)** before sending input to the program.
* Special keys like **Backspace**, **Ctrl+U**, **Ctrl+C** are handled by the **terminal driver** before your application sees them.
* Ideal for basic command-line programs like `cat`, `grep`, or `read`.

### Example:

If the user types:

```
hello<Backspace><Enter>
```

The program receives:

```
hell
```

---

## 🔸 Non-Canonical Mode (Raw Mode)

* Input is **character-buffered**:

  * Each key press is made available to the application **immediately**.
* Terminal special characters (like Backspace, Ctrl+C) are **not interpreted** automatically.
* The application must **manually handle** cursor movement, deletion, and editing.
* Enables interactive programs like shells, editors (e.g., Vim), REPLs, or games.

### Why use it?

* Needed when you want **fine control** over user input.
* Let's you implement custom line editors, keybindings, input history, etc.

### Example:

In our Rust shell:

```rust
termios.c_lflag &= !(ICANON | ECHO);
```

* `ICANON` disables canonical mode.
* `ECHO` disables automatic character echoing.

Now every keystroke triggers code like:

```rust
io::stdin().read_exact(&mut byte)
```

And we manually handle:

* Arrow keys
* Character insertion
* Backspace deletion
* Cursor movement

---

## 🔹 Escape Sequences (Arrow Keys)

When in raw mode, arrow keys don't send single-byte codes. Instead:

* `Up Arrow`: `\x1b[A`
* `Down Arrow`: `\x1b[B`
* `Right Arrow`: `\x1b[C`
* `Left Arrow`: `\x1b[D`

The application must detect these sequences and respond accordingly (e.g., fetch history or move the cursor).

---

## 🧠 Summary

| Mode          | Behavior           | When to Use                |
| ------------- | ------------------ | -------------------------- |
| Canonical     | Line-buffered      | Basic input (e.g., `read`) |
| Non-Canonical | Character-buffered | Shells, editors, REPLs     |

---

## ✅ In Our Shell

We switched to non-canonical mode with:

```rust
tcsetattr(stdin_fd, TCSANOW, &termios);
```

And read input byte-by-byte to build a custom input experience.
This gives us:

* Input history with ↑/↓
* Cursor navigation with ←/→
* Manual character insertion & deletion

It's how real shells like Bash and editors like Vim work under the hood!
