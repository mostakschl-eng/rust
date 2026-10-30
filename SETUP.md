# Rust Learning Workspace: Master Setup & Architecture Blueprint (`SETUP.md`)

Welcome to the **Rust Learning Workspace**. This document is the single source of truth for how this workspace is structured, how to recreate it on any new machine, and the rules for maintaining it over time.

---

## 1. Workspace Purpose

This repository is an isolated, modular learning environment for an absolute beginner learning Rust as their first programming language. It is designed to:
* Guide a student from "child level" (zero coding background) to "adult level" (production-ready Rust developer) across **80 granular lessons**.
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
├── ROADMAP.md                   # 80-Lesson syllabus with interactive checkboxes
│
├── guides/                      # Quick reference materials for the student
│   ├── cargo_cheatsheet.md      # Common Cargo commands explained simply
│   └── compiler_errors.md       # Rust compiler error messages explained in plain English
│
├── lessons/                     # 80 Granular Lessons across 8 Levels (Child to Adult)
│   ├── 01_baby_steps/           # Lessons 01 – 10 (Code basics, functions, variables)
│   ├── 02_types_and_math/       # Lessons 11 – 20 (Numbers, booleans, arithmetic, scope)
│   ├── 03_logic_and_loops/      # Lessons 21 – 30 (If/else, while, loop, for ranges)
│   ├── 04_collections/          # Lessons 31 – 40 (Tuples, arrays, vectors, strings, maps)
│   ├── 05_structs_and_enums/    # Lessons 41 – 50 (Structs, methods, enums, match)
│   ├── 06_ownership_and_memory/ # Lessons 51 – 60 (Stack/heap, ownership, borrowing, lifetimes)
│   ├── 07_errors_and_io/        # Lessons 61 – 70 (Option, Result, ?, stdin, files, modules)
│   └── 08_advanced_and_concurrency/ # Lessons 71 – 80 (Generics, traits, closures, threads)
│
├── tasks/                       # Hands-on practice tasks (each is an isolated Cargo package)
│   ├── 01_baby_steps/
│   │   ├── task_01_hello/
│   │   │   ├── Cargo.toml       # Cargo package configuration
│   │   │   ├── README.md        # Task objective, instructions & hidden progressive hints
│   │   │   ├── src/
│   │   │   │   └── main.rs      # Starter code where the student writes and runs code
│   │   │   └── solution/        # Dedicated anti-spoiler subfolder
│   │   │       ├── solution.rs  # Working reference solution
│   │   │       └── explain.md   # Line-by-line breakdown and beginner tips
│   │   └── ...
│   └── ...
│
└── progress/
    └── log.md                   # Chronological diary of completed milestones with copy-paste template
```

---

## 3. Role of Each Core File

| File / Folder | Role | Who Edits It? |
|---|---|---|
| `SETUP.md` | Blueprint of the entire workspace layout and reproduction steps. | AI Tutor / Developer whenever structure updates. |
| `TUTOR.md` | Guide for AI tutors (student profile, rules, live progress status). | AI Tutor after each completed lesson/task. |
| `ROADMAP.md` | Complete 80-lesson syllabus with completion checkboxes. | AI Tutor checks off items as the student finishes. |
| `guides/` | Helpful reference cheat-sheets for beginner Rust concepts and errors. | Static reference (expanded as new questions arise). |
| `lessons/` | 80 deep, conversational explanations of programming concepts. | Read by the student; authored by AI tutor. |
| `tasks/` | Standalone Cargo projects where the student types and tests code. | Student writes code in `src/main.rs`. |
| `solution/` | Isolated subfolder inside each task containing working code & notes. | Reference only; accessed when the student gets stuck or completes the task. |
| `progress/log.md` | Personal learning journal with reusable template and milestones. | Updated collaboratively after completing tasks. |

---

## 4. Maintenance Protocol (Golden Rules)

1. **Always Update `SETUP.md` When Structure Changes:**
   If a new folder, guide, or organizational convention is introduced, update the directory tree and file table in this document immediately.
2. **Never Put Solutions in the Same File as the Task:**
   To protect the student's learning experience, every task MUST have its solution isolated inside a `solution/` subfolder.
3. **Keep Every Task an Independent Cargo Package:**
   Running `cargo run` inside `tasks/.../task_name/` must work cleanly without interfering with other tasks.
4. **Keep `TUTOR.md` and `progress/log.md` Updated Live:**
   The "Where am I now?" section in `TUTOR.md` and the snapshot in `progress/log.md` must always reflect the current active lesson and task.
