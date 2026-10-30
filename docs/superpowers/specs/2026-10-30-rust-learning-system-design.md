# Rust Zero-to-Hero Learning Workspace & AI Tutor System Design

## 1. Executive Summary & Purpose

This workspace (`d:\rust-learning`) is an interactive, modular, step-by-step learning environment designed specifically for a student with **zero prior programming experience** who is learning Rust as their first language.

The workspace is designed to:
1. Provide an AI-agnostic master control file (`TUTOR.md`) that guides any AI (Antigravity, ChatGPT, Claude) on how to tutor the student, track live progress, and determine the exact next step.
2. Structure lessons into clear, simple, deep markdown guides written in plain English with real-life analogies.
3. Provide hands-on practice tasks with an anti-spoiler design (solutions separated in dedicated subfolders with line-by-line explanations).
4. Maintain a persistent progress log and a comprehensive 6-level curriculum roadmap.
5. Provide beginner-friendly reference guides (compiler error decoder, Cargo cheatsheet).

---

## 2. Student Profile & Pedagogical Guidelines

### Student Profile
* **Experience:** Absolute beginner. Never coded in any programming language before.
* **Target Language:** Rust.
* **Preferred Language of Instruction:** Simple English (short sentences, conversational, no intimidating jargon).
* **Learning Style:** Deep conceptual understanding, hands-on typing and execution, progressive hint revelation, positive reinforcement.

### Pedagogical Rules for AI Tutors
1. **One Concept at a Time:** Never combine multiple distinct concepts into one overwhelming lesson.
2. **Real-Life Analogies First:** Explain *why* something exists before showing the syntax (e.g., variables as labeled boxes, functions as reusable recipe cards).
3. **Line-by-Line Code Breakdown:** Every line and symbol (`fn`, `{}`, `println!`, `;`) must be explained clearly.
4. **Active Practice Before Progress:** The student must write and execute code in `tasks/` using `cargo run` before advancing.
5. **Anti-Spoiler Hints:** Never reveal the full code solution immediately. Offer Hint 1, then Hint 2, and direct them to the solution folder only if explicitly requested.
6. **State Tracking:** Always keep `TUTOR.md` and `progress/log.md` updated with the current level, active task, and completion status.

---

## 3. Directory Layout & Architecture

```text
d:\rust-learning\
│
├── TUTOR.md                 # Master AI brief, student profile, live state tracker
├── ROADMAP.md               # 6-level master syllabus with interactive checkboxes
│
├── guides/                  # Beginner reference materials
│   ├── compiler_errors.md   # Plain-English translation of common compiler errors
│   └── cargo_cheatsheet.md  # 4 essential Cargo commands with explanations
│
├── lessons/                 # Structured, deep lesson guides
│   ├── 01_baby_steps/
│   │   ├── 01_first_program.md
│   │   ├── 02_functions.md
│   │   ├── 03_variables_and_mutability.md
│   │   └── 04_data_types.md
│   ├── 02_logic/
│   ├── 03_data_structures/
│   ├── 04_ownership/
│   ├── 05_error_handling/
│   └── 06_projects/
│
├── tasks/                   # Hands-on practice tasks (standalone Cargo packages)
│   ├── 01_baby_steps/
│   │   ├── task_01_hello/
│   │   │   ├── Cargo.toml
│   │   │   ├── README.md         # Task goals, instructions, hidden hints
│   │   │   ├── src/
│   │   │   │   └── main.rs       # Starter code where the student writes Rust
│   │   │   └── solution/         # Separate subfolder (anti-spoiler)
│   │   │       ├── solution.rs   # Verified reference code
│   │   │       └── explain.md    # Line-by-line explanation & common pitfalls
│   │   └── task_02_greet_function/
│   │       └── ...
│
└── progress/                # Student diary & history log
    └── log.md               # Date, milestone, what was built, and reflections
```

---

## 4. Component Details & Specifications

### 4.1. `TUTOR.md` (Master AI Tutor & State Coordinator)
Must be placed at the workspace root. Any AI reading this file receives:
* Student bio, tone rules, and tutoring directives.
* **Live Status Widget:**
  * Current Level
  * Current Lesson
  * Active Practice Task
  * Next Step to unlock
* **Quick Health Checklist:** How to test the student's setup (`rustc --version`, `cargo --version`).
* **Update Protocol:** Step-by-step instructions for AI on how to update `TUTOR.md` when a task passes.

### 4.2. `ROADMAP.md` (6-Level Curriculum)
* **Level 1: Absolute Foundations** (main, println!, functions, let, mut, i32, f64, bool, char, String) -> *Milestone: Personal Profile Card*
* **Level 2: Decision Making & Repetition** (if/else, comparisons, loops, while, for ranges) -> *Milestone: Secret Number Guessing Game*
* **Level 3: Grouping Data** (Tuples, Arrays, Vectors, Structs, Enums) -> *Milestone: Student Grade Tracker*
* **Level 4: Rust's Superpower: Memory & Ownership** (Stack vs Heap, Ownership Rules, Borrowing, References `&`, Slices `&str`) -> *Milestone: Word Frequency Counter*
* **Level 5: Real-World Rust** (Option, Result, ? operator, reading user input, reading files) -> *Milestone: Persistent Todo List CLI*
* **Level 6: Capstone Project** (Building a complete CLI tool or mini HTTP server)

### 4.3. Task Standard (`tasks/XX_topic/task_YY/`)
Each task is an independent Cargo project created so the student can run `cargo run` without build conflicts.
* `README.md`: Contains Goal, Expected Output, Instructions, and expandable HTML `<details><summary>Hint 1</summary></details>`.
* `src/main.rs`: Contains initial comments and a `todo!()` or scaffold for the student.
* `solution/solution.rs`: Clean, idiomatic solution.
* `solution/explain.md`: Explains *why* the solution works and contrasts it with common mistakes.

### 4.4. Beginner Guides (`guides/`)
* `cargo_cheatsheet.md`: Focuses only on `cargo run`, `cargo check`, `cargo build`.
* `compiler_errors.md`: Breaks down Rust's friendly compiler warnings and errors into human language.

---

## 5. Verification & Testing

* Each task must compile cleanly with `cargo check` and run with `cargo run`.
* All markdown files must have accurate relative links to each other for easy navigation in VS Code / Antigravity editor.
* Initial bootstrap will include Level 1, Lesson 01, and Task 01 pre-built and verified.

