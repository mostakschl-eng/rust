# Study Tracker 2: Study Minutes

[Previous: Profile Card](../../01_baby_steps/project_01_profile_card/TASK.md)

[Next: Goal Tracker](../../03_logic_and_loops/project_03_goal_tracker/TASK.md)

## Before you start

Finish Lessons 01–20 and their numbered tasks and the previous checkpoint. This checkpoint uses only that stage plus the short checking guide introduced after Lesson 20.

These are independent snapshots of one growing project. Each folder runs by itself. Rebuild the retained features here; do not import later checkpoint code into an earlier one. Your previous solution remains available in its own folder.

## Mission

Extend the same Study Tracker by calculating the time studied and time still needed.

Retained features: learner Rahim, daily goal 90 minutes, reusable header and footer.

Read the [checking guide](../../../../guides/testing_basics.md) first. New tool: `assert_eq!(actual, expected)` is a macro (notice `!`). It checks equality while the program runs. Passing is silent; failure panics and reports the unequal values. You do not need formal unit-test syntax yet.

## Requirements

1. Retain Rahim, goal 90, the reusable header/footer, and their output.
2. Define `study_minutes(sessions: i32, minutes_each: i32) -> i32`. Return the product of its inputs.
3. Use variables for 3 sessions and 25 minutes. Call your function to calculate 75. Calculate the remaining 15 from the goal and total; do not print a hardcoded result instead of calculating.
4. Use `assert_eq!` to check total 75 and remaining 15. Also check that zero sessions, or zero minutes per session, gives zero total.
5. Use these fixed small nonnegative numbers. This checkpoint does not accept arbitrary inputs. Subtraction here assumes the total is no larger than the goal; the next checkpoint handles that boundary.
6. Print the report below. Do not use loops, keyboard input, or collections.

## Run your work

Start in the repository root (the folder containing ROADMAP.md):

```powershell
cd tasks/step_1_core_foundations/02_types_and_math/project_02_study_minutes
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
Studied: 75 minutes
Remaining: 15 minutes
Keep learning, one session at a time.
```

## Hints

<details>
<summary>Hint 1: choose the pieces</summary>

Name the session count and minutes per session separately. Multiply them inside your function.

</details>

<details>
<summary>Hint 2: connect the pieces</summary>

Return the multiplication expression without a final semicolon. In main subtract that answer from DAILY_GOAL; put the computed expression and your expected answer on opposite sides of assert_eq!.

</details>

## Acceptance checks

- Your starter work passes `cargo check`; that alone does not mean the task is solved.
- `cargo run` prints the report above, in order.
- The required assertions pass, including the listed boundary cases.
- You can explain the calculation and zero checks in your own words.

After your attempt, run `cargo run --example solution` from this task folder to compare with the separate reference. Read its [explanation](solution/explain.md) when cross-checking your work.
