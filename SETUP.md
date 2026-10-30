# Rust Learning Workspace: Master Setup & Architecture Blueprint (`SETUP.md`)

Welcome to the **Rust Learning Workspace**. This document is the single source of truth for how this workspace is structured, how to recreate it on any new machine, and the rules for maintaining it over time.

---

## 1. Workspace Purpose

This repository is an isolated, modular learning environment for an absolute beginner learning Rust as their first programming language. It is designed to:
* Keep lessons, tasks, and solutions strictly separated.
* Prevent accidental spoiler exposure (solutions live in separate subfolders).
* Enable any AI assistant (Antigravity, ChatGPT, Claude) to tutor seamlessly by reading `TUTOR.md`.
* Track progress live in `TUTOR.md` and `progress/log.md`.

---

## 2. Complete Directory Structure

```text
d:\rust-learning\
│
├── SETUP.md                     # [THIS FILE] Master blueprint & setup guide
├── TUTOR.md                     # Master AI tutor brief, student profile & live tracker
├── ROADMAP.md                   # 6-Level curriculum with interactive checkboxes
│
├── guides/                      # Quick reference materials for the student
│   ├── cargo_cheatsheet.md      # Common Cargo commands explained simply
│   └── compiler_errors.md       # Rust compiler error messages explained in plain English
│
├── lessons/                     # Step-by-step reading guides (concept by concept)
│   ├── 01_basics/               # Level 1 lessons (syntax, functions, variables, types)
│   ├── 02_logic/                # Level 2 lessons (if/else, loops, match)
│   ├── 03_data_structures/      # Level 3 lessons (tuples, arrays, vectors, structs, enums)
│   ├── 04_ownership/            # Level 4 lessons (ownership, borrowing, references, slices)
│   ├── 05_error_handling/       # Level 5 lessons (Option, Result, file I/O, user input)
│   └── 06_projects/             # Level 6 milestone project guides
│
├── tasks/                       # Hands-on practice tasks (each is an isolated Cargo package)
│   ├── 01_basics/
│   │   ├── task_01_hello/
│   │   │   ├── Cargo.toml       # Cargo package configuration
│   │   │   ├── README.md        # Task objective, instructions & hidden progressive hints
│   │   │   ├── src/
│   │   │   │   └── main.rs      # Starter code where the student writes and runs code
│   │   │   └── solution/        # Dedicated anti-spoiler subfolder
│   │   │       ├── solution.rs  # Working reference solution
│   │   │       └── explain.md   # Line-by-line breakdown and beginner tips
│   │   └── task_02_greet/       # Next task...
│   ├── 02_logic/
│   ├── 03_data_structures/
│   ├── 04_ownership/
│   ├── 05_error_handling/
│   └── 06_projects/
│
└── progress/
    └── log.md                   # Chronological diary of completed milestones
```

---

## 3. Role of Each Core File

| File / Folder | Role | Who Edits It? |
|---|---|---|
| `SETUP.md` | Blueprint of the entire workspace layout and reproduction steps. | AI Tutor / Developer whenever structure updates. |
| `TUTOR.md` | Guide for AI tutors (student profile, rules, live progress status). | AI Tutor after each completed lesson/task. |
| `ROADMAP.md` | Complete 6-level syllabus with completion checkboxes. | AI Tutor checks off items as the student finishes. |
| `guides/` | Helpful reference cheat-sheets for beginner Rust concepts and errors. | Static reference (expanded as new questions arise). |
| `lessons/` | Deep, conversational explanations of programming concepts. | Read by the student; added by the AI tutor. |
| `tasks/` | Standalone Cargo projects where the student types and tests code. | Student writes code in `src/main.rs`. |
| `solution/` | Isolated subfolder inside each task containing working code & notes. | Reference only; accessed when the student gets stuck or completes the task. |
| `progress/log.md` | Personal learning journal with dates and milestone achievements. | Updated collaboratively after completing tasks. |

---

## 4. How to Recreate This Workspace on Any Machine

If you switch computers, clone this repository, or want to start fresh, follow these steps:

### Prerequisites
Make sure Rust and Git are installed on your machine:
```bash
# Check Rust compiler
rustc --version

# Check Cargo package manager
cargo --version

# Check Git
git --version
```
*(If Rust is missing, install it from [https://rustup.rs](https://rustup.rs).)*

### Step-by-Step Recreation Commands
To recreate the directory structure from scratch using a terminal (Bash or PowerShell):

```bash
# 1. Initialize Git repository (if starting in a new folder)
git init

# 2. Create the core folders
mkdir guides
mkdir -p lessons/01_basics lessons/02_logic lessons/03_data_structures lessons/04_ownership lessons/05_error_handling lessons/06_projects
mkdir -p tasks/01_basics tasks/02_logic tasks/03_data_structures tasks/04_ownership tasks/05_error_handling tasks/06_projects
mkdir progress

# 3. Create a new task (Example: task_01_hello)
cd tasks/01_basics
cargo new task_01_hello --bin
cd task_01_hello
mkdir solution
touch solution/solution.rs solution/explain.md
```

---

## 5. Maintenance Protocol (Golden Rules)

1. **Always Update `SETUP.md` When Structure Changes:**
   If a new folder, guide, or organizational convention is introduced, update the directory tree and file table in this document immediately.
2. **Never Put Solutions in the Same File as the Task:**
   To protect the student's learning experience, every task MUST have its solution isolated inside a `solution/` subfolder.
3. **Keep Every Task an Independent Cargo Package:**
   Running `cargo run` inside `tasks/.../task_name/` must work cleanly without interfering with other tasks.
4. **Keep `TUTOR.md` Updated Live:**
   The "Where am I now?" section in `TUTOR.md` must always reflect the current active lesson and task.
