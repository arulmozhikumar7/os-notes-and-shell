
# 🧠 Understanding Processes in Operating Systems

### ✅ What is a Process?
A **process** is the execution of a program — i.e., a set of instructions loaded in memory and currently being executed by the CPU.

---

### ⚙️ Multitasking and Time Sharing

On a **single-core CPU**, only **one process can execute at a time**.  
However, modern operating systems create an illusion of multitasking through **time-sharing**.

The OS rapidly switches between processes every few milliseconds using a mechanism called **context switching**.

Since these switches happen so fast (e.g., every 5–50ms), it **feels like multiple applications are running simultaneously** — even though only one is active at any moment.

#### 🕒 Example Timeline of Time Sharing:

```text
[0ms-10ms] → Chrome runs
[10ms-20ms] → VS Code runs
[20ms-30ms] → Spotify runs
[30ms-40ms] → Chrome runs again
...and so on
````

---

### 💡 Multi-Core CPUs

Today’s systems typically use **multi-core CPUs**, where each core can execute **one process at a time**.

So with `n` cores, up to `n` processes can run **truly in parallel**.

However:

* Even with multiple cores, **multitasking still depends heavily on context switching**
* Having **too many processes** can reduce performance due to increased overhead from:

  * Context switching
  * Cache misses
  * Scheduling complexity

---

### 🔄 Process States

Processes usually exist in one of the following states:

* `Ready`: Process is waiting to be scheduled on the CPU
* `Running`: Currently being executed
* `Blocked`: Waiting for I/O or an event to continue

---

### 📦 Process Control Block (PCB)

The **PCB** is a data structure maintained by the OS that stores essential information about a process:

* Process ID (PID)
* Current state
* Program Counter
* CPU registers
* Memory boundaries
* I/O status

---

### 🧠 Context Switching

During a **context switch**:

* The OS saves the current process's context (registers, program counter, etc.) into its PCB
* Then loads the next process's context from its PCB into the CPU
* This allows resuming from exactly where it left off

---

### 🧰 Process API (System Calls)

Operating systems provide APIs to manage processes:

* Create a new process
* Destroy a process
* Wait for a process to finish
* Suspend/resume a process
* Check process status

In Unix-like systems, these are exposed via system calls like `fork()`, `exec()`, `wait()`, and `exit()`.

---

### 🧱 Memory Layout – Stack vs Heap

When a process runs, the OS sets up memory regions including the **stack** and **heap**.

* **Stack**:

  * Used for local variables, function calls, return addresses
  * Fixed-size and follows LIFO (Last In First Out)
  * Automatically managed

* **Heap**:

  * Used for dynamic memory allocation (e.g., linked lists, trees, objects)
  * Manually managed (in C/C++) or by a Garbage Collector (in Java)

#### 🔍 Example in C:

```c
#include <stdio.h>
#include <stdlib.h>

void example() {
    int x = 10; // stored on stack
    int* ptr = (int*)malloc(sizeof(int)); // stored on heap
    *ptr = 20;
    printf("x = %d, *ptr = %d\n", x, *ptr);
    free(ptr); // deallocate heap memory
}

int main() {
    example();
    return 0;
}
```

* `x` is a **stack variable**
* `ptr` points to a memory location in the **heap**
* Manual memory management is required via `malloc` and `free`

---

### 🗑️ Memory Management in Different Languages

* **Java**: Has a built-in **Garbage Collector** (GC) that handles memory allocation and deallocation automatically
* **C++**: Memory is usually managed manually using `new`/`delete`, or via **smart pointers** (`std::unique_ptr`, `std::shared_ptr`)
* Garbage collection **is not built-in** to C++, but it can be added via external libraries (rare in practice)
