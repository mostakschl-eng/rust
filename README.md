# 🦀 Rust Zero-to-Hero: Child to Adult Developer Curriculum

> **A Rust learning workspace for someone starting from zero.**  
> Step 1 contains 80 lessons and matching exercises across eight levels. Steps 2–4 are outlines for future study; completing Step 1 is a foundation for building projects, not a guarantee of production experience.

[![GitHub Repo](https://img.shields.io/badge/GitHub-mostakschl--eng%2Frust-blue?logo=github)](https://github.com/mostakschl-eng/rust)
[![Rust Version](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Lessons](https://img.shields.io/badge/Lessons-80-brightgreen)](ROADMAP.md)
[![Tasks](https://img.shields.io/badge/Practice%20Tasks-80-success)](tasks/step_1_core_foundations/README.md)

---

## 🗺️ The 4-Stage Learning Roadmap

The roadmap has four stages. Only Step 1 currently contains full lesson and task packages:

```text
rust-learning/
│
├── Step 1: Core Foundations (80 Lessons) ──► 8 Levels: Syntax, Memory, Ownership, Concurrency
│
├── Step 2: Universal Production Crates   ──► serde (JSON), tokio (Async), anyhow (Errors)
│
├── Step 3: Specialization Career Tracks  ──► Web APIs (axum), CLI (clap), Systems (Wasm/gRPC)
│
└── Step 4: Portfolio Projects & CI/CD    ──► 3 Full Apps, clippy, unit testing, GitHub Actions
```

---

## 📚 Step 1: The 8-Level Core Curriculum (80 Lessons)

| Level | Topic | Description | Lessons | Practice Tasks |
|:---:|---|---|:---:|:---:|
| **Level 1** | 🟢 **Baby Steps** | `fn main`, `println!`, variables, `mut`, functions, recipes | [Lessons 01 – 10](lessons/step_1_core_foundations/01_baby_steps/) | [Tasks 01 – 10](tasks/step_1_core_foundations/01_baby_steps/) |
| **Level 2** | 🟡 **Types & Math** | Integers (`i32`), floats (`f64`), bool, char, operators, casting | [Lessons 11 – 20](lessons/step_1_core_foundations/02_types_and_math/) | [Tasks 11 – 20](tasks/step_1_core_foundations/02_types_and_math/) |
| **Level 3** | 🟠 **Logic & Loops** | `if`/`else`, `loop`, `while`, `for`, ranges, break with values | [Lessons 21 – 30](lessons/step_1_core_foundations/03_logic_and_loops/) | [Tasks 21 – 30](tasks/step_1_core_foundations/03_logic_and_loops/) |
| **Level 4** | 🔵 **Collections** | Tuples, arrays, `Vec<T>`, slices, `String` vs `&str`, `HashMap` | [Lessons 31 – 40](lessons/step_1_core_foundations/04_collections/) | [Tasks 31 – 40](tasks/step_1_core_foundations/04_collections/) |
| **Level 5** | 🟣 **Structs & Enums** | Named structs, methods (`impl`), enums, optionals, `match` | [Lessons 41 – 50](lessons/step_1_core_foundations/05_structs_and_enums/) | [Tasks 41 – 50](tasks/step_1_core_foundations/05_structs_and_enums/) |
| **Level 6** | 🔴 **Ownership Core** | Stack vs Heap, 3 Ownership rules, Move vs Copy, Borrowing, Lifetimes | [Lessons 51 – 60](lessons/step_1_core_foundations/06_ownership_and_memory/) | [Tasks 51 – 60](tasks/step_1_core_foundations/06_ownership_and_memory/) |
| **Level 7** | 🟤 **Errors & I/O** | Eliminating NULL, `Option<T>`, `Result<T,E>`, `?` operator, File I/O | [Lessons 61 – 70](lessons/step_1_core_foundations/07_errors_and_io/) | [Tasks 61 – 70](tasks/step_1_core_foundations/07_errors_and_io/) |
| **Level 8** | 🏆 **Advanced Mastery** | Generics, Traits, Closures, Iterators, `Box<T>`, Threads, Channels | [Lessons 71 – 80](lessons/step_1_core_foundations/08_advanced_and_concurrency/) | [Tasks 71 – 80](tasks/step_1_core_foundations/08_advanced_and_concurrency/) |

*For the complete detailed checklist of all 80 individual lessons, check out [`ROADMAP.md`](ROADMAP.md).*

---

## 📂 Repository File Structure

```text
d:\rust-learning/
│
├── README.md                                 # Main project overview & quickstart guide
├── ROADMAP.md                                # Master 80-lesson interactive checklist
├── TUTOR.md                                  # AI Tutor protocol & live state tracker
├── AGENTS.md                                 # Strict AI assistant rules (No Git touch)
│
├── lessons/                                  # Deep, beginner-friendly conceptual lessons
│   ├── step_1_core_foundations/              # 80 Granular Lessons across 8 Levels
│   │   ├── 01_baby_steps/                    # Lessons 01 – 10 (fn main, println!, variables)
│   │   ├── 02_types_and_math/                # Lessons 11 – 20 (i32, f64, bool, casting, errors)
│   │   ├── 03_logic_and_loops/               # Lessons 21 – 30 (if/else, loop, while, for)
│   │   ├── 04_collections/                   # Lessons 31 – 40 (arrays, vectors, strings, hash maps)
│   │   ├── 05_structs_and_enums/             # Lessons 41 – 50 (structs, methods, enums, match)
│   │   ├── 06_ownership_and_memory/          # Lessons 51 – 60 (Stack/Heap, Move, Borrow, Lifetimes)
│   │   ├── 07_errors_and_io/                 # Lessons 61 – 70 (No NULL, Option, Result, ?, File I/O)
│   │   └── 08_advanced_and_concurrency/      # Lessons 71 – 80 (Generics, Traits, Threads, Channels)
│   ├── step_2_universal_crates/              # serde, tokio, anyhow lessons
│   ├── step_3_specialization_tracks/         # axum (Web), clap (CLI), systems (Wasm)
│   └── step_4_production_and_projects/       # Real apps & quality standards
│
├── tasks/                                    # Hands-on practice projects (Cargo packages)
│   ├── step_1_core_foundations/              # 80 Matching Task Folders
│   │   ├── 01_baby_steps/                    # Tasks 01 – 10
│   │   ├── ...                               # Tasks 11 – 70
│   │   └── 08_advanced_and_concurrency/      # Tasks 71 – 80
│   ├── step_2_universal_crates/              # Practice with external crates
│   ├── step_3_specialization_tracks/         # Track practice packages
│   └── step_4_production_and_projects/       # Standalone portfolio project repositories
│
└── progress/
    └── log.md                                # Student personal learning diary & milestone log
```

---

## 🔒 The Anti-Spoiler Task System

Each Step 1 task is an independent Cargo project. Write your answer in `src/main.rs`; use the separate solution only when you need help or want to compare your finished work:

```text
task_01_hello/
├── Cargo.toml               # Standalone manifest; solution is a separate example target
├── TASK.md                  # Instructions, requirements & expected terminal output
├── src/
│   └── main.rs              # TODO starter (Task 20 has intentional compiler errors)
└── solution/                # Anti-Spoiler Subfolder
    ├── solution.rs          # Verified complete solution code
    └── explain.md           # Step-by-step line-by-line explanation
```

---

## 🚦 How to Start Learning

1. **Clone or Open Workspace:**
   Follow [SETUP.md](SETUP.md) to install Rust, then open this folder in your editor.
2. **Open Lesson 01:**
   Read [`lessons/step_1_core_foundations/01_baby_steps/01_what_is_code.md`](lessons/step_1_core_foundations/01_baby_steps/01_what_is_code.md).
3. **Open Practice Task 01:**
   Read [`tasks/step_1_core_foundations/01_baby_steps/task_01_hello/TASK.md`](tasks/step_1_core_foundations/01_baby_steps/task_01_hello/TASK.md).
4. **Write Code in `src/main.rs` & Run:**
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_01_hello
   cargo run
   ```
5. **Check Solution (Only if Stuck):**
   Compare your work with [`solution/solution.rs`](tasks/step_1_core_foundations/01_baby_steps/task_01_hello/solution/solution.rs).
   From the task folder, `cargo run --example solution` runs the reference answer without replacing your own code. Try the task and its hints before opening it.
6. **Mark Progress:**
   Check off `[x]` in [`ROADMAP.md`](ROADMAP.md) and record notes in [`progress/log.md`](progress/log.md).

---

## 🤖 AI Tutor Protocol (`TUTOR.md`)

This workspace includes an AI Teaching Protocol. Any AI assistant reading [`TUTOR.md`](TUTOR.md) will:
* Teach in simple English with real-life analogies.
* Never spoil answers upfront (offers Hints first).
* Keep track of your progress and guide you lesson-by-lesson.

---

## 🌐 Connecting to GitHub

This repository is configured to push to:
👉 **[https://github.com/mostakschl-eng/rust](https://github.com/mostakschl-eng/rust)**

Whenever you want to save and push your progress:
```bash
git add .
git commit -m "Update Rust learning progress"
git push -u origin master
```

---

## Content status
Step 1 contains the core lessons and practice exercises. Later-stage folders describe planned topics. A license file has not been supplied in this repository.
