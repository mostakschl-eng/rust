# Study Tracker 3: Goal Tracker

[Previous: Study Minutes](../../02_types_and_math/project_02_study_minutes/TASK.md)

[Next: Session Collection](../../04_collections/project_04_session_collection/TASK.md)

## Before you start

Finish Lessons 01–30 and their numbered tasks and the previous checkpoint. This checkpoint uses only that stage plus the short checking guide introduced after Lesson 20.

These are independent snapshots of one growing project. Each folder runs by itself. Rebuild the retained features here; do not import later checkpoint code into an earlier one. Your previous solution remains available in its own folder.

## Mission

Extend the same Study Tracker by repeating session work and classifying progress.

Retained features: learner Rahim, daily goal 90 minutes, reusable header and footer.

Read the [checking guide](../../../../guides/testing_basics.md) first. New tool: `assert_eq!(actual, expected)` is a macro (notice `!`). It checks equality while the program runs. Passing is silent; failure panics and reports the unequal values. You do not need formal unit-test syntax yet.

## Requirements

1. Retain Rahim, goal 90, the reusable header/footer, the session summary, total 75, and remaining 15.
2. Start a mutable total at zero. Use one `for` loop over `1..=sessions` with sessions 3 and minutes_each 25. Add 25 each time and print that session. Calculate the displayed total through this loop.
3. Define `progress_level(total: i32, goal: i32) -> i32`. Return 0 below the goal, 1 exactly at it, and 2 above it. These are temporary category codes, not minute amounts. Use `if / else if / else` to choose.
4. Define `remaining_minutes(total: i32, goal: i32) -> i32`. Return the positive shortfall below the goal and zero at or above it.
5. Use the progress code to print `Progress: below goal`, `Progress: goal reached`, or `Progress: above goal`.
6. Assert progress codes for totals 0, 89, 90, 91 against goal 90. Assert remaining minutes for totals 0, 75, 90, 91 are 90, 15, 0, 0. Assert that the loop total is 75.
7. Keep the default data fixed and small. Do not add keyboard input, randomness, collections, or new data types.

## Run your work

Start in the repository root (the folder containing ROADMAP.md):

```powershell
cd tasks/step_1_core_foundations/03_logic_and_loops/project_03_goal_tracker
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
Sessions: 3 x 25 minutes
Session 1: 25 minutes
Session 2: 25 minutes
Session 3: 25 minutes
Studied: 75 minutes
Remaining: 15 minutes
Progress: below goal
Keep learning, one session at a time.
```

## Hints

<details>
<summary>Hint 1: choose the pieces</summary>

The total must survive across loop turns. Declare it before the loop using let mut.

</details>

<details>
<summary>Hint 2: connect the pieces</summary>

Use an inclusive range so the third session runs. Check less-than first, equality second, and use else for above. Remaining time becomes zero once the goal is met.

</details>

## Acceptance checks

- Your starter work passes `cargo check`; that alone does not mean the task is solved.
- `cargo run` prints the report above, in order.
- The required assertions pass, including the listed boundary cases.
- You can explain the loop total and three progress categories in your own words.

After your attempt, run `cargo run --example solution` from this task folder to compare with the separate reference. Read its [explanation](solution/explain.md) when cross-checking your work.
