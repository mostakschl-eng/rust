# Study Tracker 1: Profile Card

Previous: none; this is the first checkpoint.

[Next: Study Minutes](../../02_types_and_math/project_02_study_minutes/TASK.md)

## Before you start

Finish Lessons 01–10 and their numbered tasks. This checkpoint uses only Lessons 01–10.

These are independent snapshots of one growing project. Each folder runs by itself. Rebuild the retained features here; do not import later checkpoint code into an earlier one. Your previous solution remains available in its own folder.

## Mission

Build the first Study Tracker screen: a learner profile with a daily goal.

Retained features: none yet; this is the first snapshot.

## Requirements

1. Store Rahim in a variable created with `let`.
2. Define `const DAILY_GOAL: i32` with value 90. Here `i32` means a signed 32-bit whole number; Lesson 10 uses explicit constant types.
3. Define `print_header()` and `print_footer()` functions. Call them around the profile lines.
4. Print the learner and goal using `{}` placeholders. Match the output below.
5. Use only Lessons 01–10. Do not add input, loops, or collections.

## Run your work

Start in the repository root (the folder containing ROADMAP.md):

```powershell
cd tasks/step_1_core_foundations/01_baby_steps/project_01_profile_card
cargo check
cargo run
```

The starter intentionally compiles without producing the finished report. Replace the TODO work yourself.

## Expected stdout

Captured from the reference example. Cargo build messages and warnings are not part of this program output. No keyboard input is needed.

```text
=== Study Tracker ===
Learner: Rahim
Daily goal: 90 minutes
Keep learning, one session at a time.
```

## Hints

<details>
<summary>Hint 1: choose the pieces</summary>

Treat the header and footer like two recipe cards. Define each once, then call it from main.

</details>

<details>
<summary>Hint 2: connect the pieces</summary>

Place the constant outside main. Bind the name inside main. Print in the order shown; each function call happens where you write it.

</details>

## Acceptance checks

- Your starter work passes `cargo check`; that alone does not mean the task is solved.
- `cargo run` prints the report above, in order.
- Your code uses a variable, a constant, and called header/footer functions.
- You can explain the execution order in your own words.

After your attempt, run `cargo run --example solution` from this task folder to compare with the separate reference. Read its [explanation](solution/explain.md) when cross-checking your work.
