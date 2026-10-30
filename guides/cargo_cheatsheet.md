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
# From the repository root, navigate to Task 01:
cd tasks/step_1_core_foundations/01_baby_steps/task_01_hello
cargo run
```

If your terminal is already inside that task, run only `cargo run`. An unfinished starter can compile without completing the requirements. Task 20 deliberately contains compiler errors for practice.

## Checking the Separate Solution

Try the task and its hints first. When you choose to check the reference answer, run this from the task folder:

```text
cargo run --example solution
```

This runs `solution/solution.rs` without replacing your own `src/main.rs`. Read `solution/explain.md` to understand why it works. `cargo check --example solution` checks the reference code without running it.

The task's expected-output block shows output from the program. Cargo also prints build messages, and the compiler may print warnings about unfinished or demonstration code. Those messages are separate from your program's output. A warning does not prevent compilation; an error does. Task 19 also uses `dbg!`, whose diagnostic goes to the separate standard-error stream.

