# Rust Learning Workspace: Setup and Structure

This workspace teaches Rust as a first programming language. Step 1 has 80 lessons across eight levels. Steps 2–4 contain outlines for later study.

## Install Rust first

1. Open the [official Rust installer page](https://rust-lang.org/tools/install/).
2. On Windows, download and run `rustup-init.exe`. Follow its prompts to install the stable toolchain. If it asks for Visual Studio C++ build tools, install the required MSVC tools and Windows SDK. These supply the linker that joins the compiled program into an executable.
3. Close and reopen your terminal after installation. On macOS or Linux, follow the commands on that same official page; you may also need the platform's C compiler/linker.
4. Check the installation in a terminal:

```text
rustc --version
cargo --version
```

Both commands should print a version number. The exact numbers change as Rust is updated. If either command is unknown, restart the terminal and follow the [official installation troubleshooting](https://doc.rust-lang.org/book/ch01-01-installation.html#troubleshooting).

`rustc` is the Rust compiler: it translates Rust code. `cargo` manages the package and asks the compiler to build it. A terminal is the window where you type commands. An editor is where you edit files.

| Item | Plain meaning |
|---|---|
| `cd folder` | Change the terminal's current folder |
| `Cargo.toml` | The package's name and build settings |
| `src/main.rs` | Your program and unfinished exercise |
| `cargo check` | Check the program's types and syntax without running it |
| `cargo run` | Compile and run your own program |
| `cargo run --example solution` | Run the separate reference answer when you choose to check it |

## Run your first task

Open this repository in your editor. In Windows PowerShell, start at its actual location (change the first path if yours is different):

```powershell
cd D:/rust-learning
cd tasks/step_1_core_foundations/01_baby_steps/task_01_hello
cargo run
```

The starter currently prints `Hello, World!`. Read `TASK.md`, write your own answer in `src/main.rs`, and run it again. The requested answer has two lines. A successful compile alone does not finish the exercise.

If Cargo says it cannot find `Cargo.toml`, the terminal is in the wrong folder. Open the terminal in the task folder, or return to the repository root before using a full `cd tasks/...` command. Do not run `cargo new` inside an existing task.

## Learn, try, then check

1. Read the matching lesson under `lessons/step_1_core_foundations/`.
2. Read `TASK.md` and try the `// TODO` work in `src/main.rs`.
3. Use `cargo check` while writing and `cargo run` to inspect the result.
4. If stuck, reveal Hint 1 before Hint 2. Explain what each hint means before continuing.
5. After trying, compare with `solution/solution.rs` and read `solution/explain.md`. `cargo run --example solution` runs that answer without overwriting your code. For interactive exercises, type the sample input when asked.
6. Explain the idea in your own words, then update the existing progress tracker when you have actually completed the task.

Task 20 intentionally starts with compiler errors. Its purpose is to read and repair those errors. All other starter packages should compile, but their unfinished code is not expected to match the completed output yet.

## Directory structure

```text
rust-learning/
├── README.md
├── SETUP.md
├── ROADMAP.md
├── TUTOR.md
├── AGENTS.md
├── guides/
│   ├── cargo_cheatsheet.md
│   └── compiler_errors.md
├── lessons/
│   ├── step_1_core_foundations/
│   │   ├── 01_baby_steps/
│   │   ├── 02_types_and_math/
│   │   ├── 03_logic_and_loops/
│   │   ├── 04_collections/
│   │   ├── 05_structs_and_enums/
│   │   ├── 06_ownership_and_memory/
│   │   ├── 07_errors_and_io/
│   │   └── 08_advanced_and_concurrency/
│   ├── step_2_universal_crates/           # Outline
│   ├── step_3_specialization_tracks/      # Outline
│   └── step_4_production_and_projects/    # Outline
├── tasks/
│   ├── step_1_core_foundations/           # Same eight level folders
│   │   └── 01_baby_steps/
│   │       └── task_01_hello/
│   │           ├── Cargo.toml
│   │           ├── Cargo.lock
│   │           ├── TASK.md
│   │           ├── src/main.rs
│   │           └── solution/
│   │               ├── solution.rs
│   │               └── explain.md
│   ├── step_2_universal_crates/           # Outline
│   ├── step_3_specialization_tracks/      # Outline
│   └── step_4_production_and_projects/    # Outline
├── docs/superpowers/                     # Design and maintenance plans
└── progress/log.md
```

## Maintaining the workspace

Keep the lesson order, independent task packages, and separate solutions. Every lesson needs simple explanations, an analogy, a symbol table, code, expected output, and common mistakes. Every task needs instructions, expected output, progressive hints, a TODO starter, a complete reference solution, and an explanation.

Use relative Markdown links so the workspace works in another folder and on GitHub. Commands that start with `cd tasks/...` assume the terminal starts at the repository root. Each manifest exposes `solution/solution.rs` as the example named `solution`; the normal `cargo run` still runs the student's `src/main.rs`.

Verify student work before updating `TUTOR.md`, `ROADMAP.md`, or `progress/log.md`. Maintenance edits do not count as lessons completed. The student manages Git; agents must follow [AGENTS.md](AGENTS.md).
