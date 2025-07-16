# 🧠 Process Control

## 🧪 fork()

- Creates a **new process** (child) from the current one (parent).
- Returns:
  - `0` in the child
  - Child’s PID in the parent

> Both child and parent resume execution **right after the `fork()` call** — but as **separate processes**.

---

## 🔄 wait() / waitpid()

- Used by a **parent process** to **pause and wait** until a child finishes.
- Prevents the creation of **zombie processes**.
- `waitpid()` allows waiting for a specific child.

```c
int status;
wait(&status);
````

---

## 🔁 exec() — Replace Current Process

* Replaces the current process’s code, stack, heap with a **new program**
* **PID stays the same**
* Used after `fork()` to run a new command in the child

### 🚀 Variants of exec()

| Variant    | Name          | Path Search? | Env Customizable? | Args Format         |
| ---------- | ------------- | ------------ | ----------------- | ------------------- |
| `execl()`  | List          | ❌ No         | ❌ No              | List of args        |
| `execv()`  | Vector        | ❌ No         | ❌ No              | Array of args       |
| `execlp()` | List + PATH   | ✅ Yes        | ❌ No              | List of args        |
| `execvp()` | Vector + PATH | ✅ Yes        | ❌ No              | Array of args       |
| `execle()` | List + Env    | ❌ No         | ✅ Yes             | List of args + env  |
| `execve()` | Vector + Env  | ❌ No         | ✅ Yes             | Array of args + env |

---

## 🚨 Signals

Signals are **asynchronous software interrupts** used to notify or control processes.

| Signal    | Purpose                   | Default Action   |
| --------- | ------------------------- | ---------------- |
| `SIGINT`  | Interrupt (Ctrl+C)        | Terminate        |
| `SIGTERM` | Terminate (gracefully)    | Terminate        |
| `SIGKILL` | Force Kill (can’t ignore) | Kill immediately |
| `SIGSTOP` | Pause execution           | Stop             |
| `SIGCONT` | Resume execution          | Continue         |
| `SIGCHLD` | Child has exited/stopped  | Notify parent    |
| `SIGUSR1` | User-defined signal 1     | Terminate        |
| `SIGUSR2` | User-defined signal 2     | Terminate        |

> Many signals can be **caught or ignored**, except `SIGKILL` and `SIGSTOP`.

---

## 🧟 Zombie Process

* A **child process that has exited**, but its **parent has not called `wait()`**.
* Still occupies an entry in the process table.
* Shows up in `ps` as `<defunct>`.

```c
pid_t pid = fork();
if (pid == 0) exit(0); // child exits
else sleep(30);        // parent doesn’t wait
```

---

## 👶 Orphan Process

* A **child still running** after the **parent has exited**.
* Automatically adopted by **init (PID 1)** or **systemd**.
* Continues running normally.
* When it finishes, it is **cleaned up by init** — no zombie.

---

## 👥 Process Ownership and Permissions

* Each process is owned by a **user ID (UID)**.
* A user can **only control their own processes**.
* Attempting to `kill` or debug another user's process will fail.

### 👑 Superuser (root)

* UID `0`
* Can control **all processes** (kill, signal, trace, etc.)
* Must be used **carefully** for **security and stability**

```bash
sudo kill -9 <pid>  # Only root can force kill other users’ processes
```

---

### ✅ Summary

* `fork()` → create new process
* `exec()` → replace current process
* `wait()` → reap children
* **Signals** → communicate control
* **Zombies** = child finished, parent didn't wait
* **Orphans** = child alive, parent is gone
* Process control is limited by **user permissions**


