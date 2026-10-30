# Tasks — Step 1: Core Rust Foundations

This directory contains the hands-on practice projects for all 80 lessons in Step 1.

Each task folder is a complete, self-contained Cargo package:
* `TASK.md`: Instructions, expected output, and step-by-step guidance.
* `src/main.rs`: Starter code with `// TODO` markers for you to solve.
* `solution/`: Anti-spoiler subfolder containing:
  - `solution.rs`: Complete working code.
  - `explain.md`: Explanation of the solution.

Run your own work with `cargo run` from the task folder. When you choose to check the answer, run `cargo run --example solution`; this does not overwrite `src/main.rs`.

Most starters compile but are intentionally unfinished. Task 20 instead contains three intentional compiler errors: use `cargo check` to inspect them, repair them, and then run your answer. Exact diagnostic wording can vary with the Rust version. Printed data must match the requirements; thread ordering and operating-system error messages may vary where noted.

---

## 📂 Subdirectories
* [`01_baby_steps/`](01_baby_steps/) — Tasks 01 to 10
* [`02_types_and_math/`](02_types_and_math/) — Tasks 11 to 20
* [`03_logic_and_loops/`](03_logic_and_loops/) — Tasks 21 to 30
* [`04_collections/`](04_collections/) — Tasks 31 to 40
* [`05_structs_and_enums/`](05_structs_and_enums/) — Tasks 41 to 50
* [`06_ownership_and_memory/`](06_ownership_and_memory/) — Tasks 51 to 60
* [`07_errors_and_io/`](07_errors_and_io/) — Tasks 61 to 70
* [`08_advanced_and_concurrency/`](08_advanced_and_concurrency/) — Tasks 71 to 80

## Connected milestones and checking

Build one Study Tracker through eight milestones, one after each level. See the [checkpoint map](../../README.md#one-study-tracker-eight-checkpoints). These independent snapshots keep earlier answers runnable and copyable. The final checkpoint combines earlier features with a threaded report.

Read [Testing basics, Part 1](../../guides/testing_basics.md#part-1-after-lesson-20) after Lesson 20 for runtime assertions. After Lesson 70, read [Part 2](../../guides/testing_basics.md#part-2-after-lesson-70-modules) for unit-test syntax. Start checking results during Step 1.
