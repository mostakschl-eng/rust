# Rust Zero-to-Hero Learning Workspace Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a modular, anti-spoiler, AI-tutor-ready Rust learning workspace in `d:\rust-learning` with master setup guide, curriculum roadmap, beginner guides, progress tracker, and initial runnable lesson/task.

**Architecture:** A lightweight root coordinator (`TUTOR.md`), root blueprint (`SETUP.md`), curriculum (`ROADMAP.md`), dedicated reference materials (`guides/`), modular lesson guides (`lessons/`), and independent runnable Cargo task packages with isolated solution folders (`tasks/`).

**Tech Stack:** Rust 1.98+, Cargo, Markdown.

**Spec:** `docs/superpowers/specs/2026-10-30-rust-learning-system-design.md`

## Global Constraints

* Language of instruction: Simple English (short sentences, beginner-friendly, real-life analogies).
* No spoilers: Task problem and solution must never share the same file; solutions reside in `solution/`.
* Cargo compatibility: Every task must be a runnable, isolated Cargo project.
* Maintenance rule: Any change to workspace directory layout must be reflected in `SETUP.md`.

---

### Task 1: Create `SETUP.md` Blueprint & Maintenance Guide

**Files:**
- Create: `SETUP.md`

- [ ] **Step 1: Write `SETUP.md`**
Define complete directory tree, role of each file, command instructions to recreate workspace on any computer, and maintenance rules.
- [ ] **Step 2: Commit**
`git add SETUP.md && git commit -m "docs: add workspace setup and structure blueprint"`

---

### Task 2: Create Master AI Coordinator (`TUTOR.md`)

**Files:**
- Create: `TUTOR.md`

- [ ] **Step 1: Write `TUTOR.md`**
Define student profile (first-time coder, Rust learner), pedagogical rules for any AI, live state tracker ("Where am I now?"), and update protocol.
- [ ] **Step 2: Commit**
`git add TUTOR.md && git commit -m "docs: add master tutor coordinator and state tracker"`

---

### Task 3: Create Master Curriculum Roadmap (`ROADMAP.md`)

**Files:**
- Create: `ROADMAP.md`

- [ ] **Step 1: Write `ROADMAP.md`**
Define all 6 levels from Absolute Foundations to Capstone Project with milestone deliverables and interactive checkboxes.
- [ ] **Step 2: Commit**
`git add ROADMAP.md && git commit -m "docs: add 6-level interactive curriculum roadmap"`

---

### Task 4: Create Beginner Guides (`guides/`)

**Files:**
- Create: `guides/cargo_cheatsheet.md`
- Create: `guides/compiler_errors.md`

- [ ] **Step 1: Write `guides/cargo_cheatsheet.md`**
Explain `cargo run`, `cargo check`, `cargo build` in plain terms.
- [ ] **Step 2: Write `guides/compiler_errors.md`**
Translate common beginner compiler errors (type mismatch, unused variables, mutability warnings) into friendly plain English.
- [ ] **Step 3: Commit**
`git add guides/ && git commit -m "docs: add cargo cheatsheet and compiler error decoder"`

---

### Task 5: Initialize Progress Diary & Directory Scaffolding

**Files:**
- Create: `progress/log.md`
- Create directory trees for `lessons/` and `tasks/`

- [ ] **Step 1: Write `progress/log.md`**
Create the initial log entry documenting the creation of the workspace.
- [ ] **Step 2: Scaffold directory tree**
Create directory folders for `lessons/01_basics` through `06_projects` and `tasks/01_basics` through `06_projects`.
- [ ] **Step 3: Commit**
`git add progress/ && git commit -m "feat: initialize progress diary and directory layout"`

---

### Task 6: Author Level 1, Lesson 01: First Program

**Files:**
- Create: `lessons/01_basics/01_first_program.md`

- [ ] **Step 1: Write `lessons/01_basics/01_first_program.md`**
Explain computers, instructions, `fn main()`, `{ }`, `println!()`, and semicolons `;` in clear, conversational Simple English.
- [ ] **Step 2: Commit**
`git add lessons/ && git commit -m "docs: add lesson 01 - first program"`

---

### Task 7: Build & Verify Task 01: Hello World

**Files:**
- Create: `tasks/01_basics/task_01_hello/Cargo.toml`
- Create: `tasks/01_basics/task_01_hello/README.md`
- Create: `tasks/01_basics/task_01_hello/src/main.rs`
- Create: `tasks/01_basics/task_01_hello/solution/solution.rs`
- Create: `tasks/01_basics/task_01_hello/solution/explain.md`

- [ ] **Step 1: Scaffold Task 01 with Cargo manifest and README**
Write `Cargo.toml`, `README.md` with goals and hidden hints.
- [ ] **Step 2: Write starter code in `src/main.rs`**
Provide clean template with `// TODO` for the student.
- [ ] **Step 3: Write isolated solution and explanation**
Write `solution/solution.rs` and `solution/explain.md`.
- [ ] **Step 4: Verify task compilation**
Run `cargo check` in `tasks/01_basics/task_01_hello/` to guarantee no syntax errors.
- [ ] **Step 5: Commit**
`git add tasks/ && git commit -m "feat: add task 01 hello world with isolated solution"`
