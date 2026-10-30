# Beginner Cargo Cheatsheet (`cargo_cheatsheet.md`)

## What is Cargo?
Cargo is Rust’s all-in-one assistant. It downloads libraries, compiles your code into a runnable program, and runs it for you.

You only need **4 simple commands** to get started:

---

### 1. `cargo run` (The Most Used Command)
Compiles your code and runs it immediately on your screen.
```bash
cargo run
```
* **When to use:** Whenever you write code in a task and want to see the output.

---

### 2. `cargo check` (The Fast Spell-Checker)
Quickly scans your code for errors without taking time to build the full program.
```bash
cargo check
```
* **When to use:** When you are typing code and want to see if the Rust compiler agrees with your syntax without waiting for a full build.

---

### 3. `cargo fmt` (The Auto-Beautifier)
Automatically formats your code: fixes spacing, indentation, and makes it look professional.
```bash
cargo fmt
```
* **When to use:** Whenever your code looks messy or unaligned.

---

### 4. `cargo build` (Make the Final Executable)
Compiles your project into a standalone executable file (`.exe` on Windows) inside the `target/debug/` folder.
```bash
cargo build
```
* **When to use:** When your program is finished and you want an executable file you can run anytime.

---

## Tip for Running Tasks
Always open your terminal inside the specific task folder before running Cargo:
```bash
# Example: Navigating to Task 01
cd tasks/01_basics/task_01_hello
cargo run
```

