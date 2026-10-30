# Study Tracker — Checkpoint 6: Read reports without taking records

## Before you start

Finish Lessons 01–60 and their numbered tasks first. Review [the relevant lesson](../../../../lessons/step_1_core_foundations/06_ownership_and_memory/59_slices_deep_dive.md) and the [checking guide](../../../../guides/testing_basics.md).

[Previous checkpoint](../../05_structs_and_enums/project_05_session_models/TASK.md) · [Next checkpoint](../../07_errors_and_io/project_07_saved_tracker/TASK.md)

This is one growing Study Tracker. Each folder is a separate, runnable snapshot. Repeated code is intentional: do not import a later checkpoint into an earlier one. Rebuild the retained features here so this folder works alone.

## What stays

Keep the learner profile, daily goal, session list, total study minutes, remaining minutes, and goal-progress decision. These are connected missions, not unrelated apps.

## Your new mission

Keep the named session model, but let report helpers borrow slices. Find the longest subject by returning a view into the existing text.

A slice `&[Session]` is a borrowed view of adjacent records. It can view a vector, array, or part of either. `Option<&str>` is `Some(borrowed_text)` or `None`; it lets an empty list have an honest answer. `Option` appeared with vector operations, and matching it is now familiar. The function has one input lifetime, so Rust's lifetime elision connects its returned borrow to that input. You do not need to write an explicit `'a` here.

## Requirements

1. Retain the Session/StudyKind model, constructor, description method, Rahim's profile, the goal of 90, and the three canonical records.
2. Change the total helper to `total_minutes(sessions: &[Session]) -> u64`. Use a loop and convert each minute value before adding it.
3. Write `longest_subject(sessions: &[Session]) -> Option<&str>`. Return `None` for no records. For a nonempty list, return a borrowed subject; do not clone or create a replacement subject.
4. Define longest as the largest `.chars().count()`: the number of Unicode scalar values. On equal counts, keep the first record. A present empty subject returns `Some("")`, which differs from no records.
5. Retain progress and remaining calculations. Print the longest subject using `match`, then print the original vector's length to show it is still available.
6. Add silent assertions for the canonical report, empty slice, a sub-slice totaling 55, a present empty subject, Unicode, ties, and goal boundaries. Use subjects `éé`, `abc`, `xyz` in this order: `abc` must win.
7. Keep the simple loops. The specific `.chars().count()` operation counts text; general iterator chains wait until Lesson 77. No input or files yet.

This is a small fixed-data exercise. The supplied records are bounded. A `u64` accumulator avoids adding the sample's `u32` minute values in a narrower type; it is not a promise of unlimited totals.

## Work and run

Edit [src/main.rs](src/main.rs). It deliberately compiles before the work is finished; its current output is not the target output.

Starting at the repository root `D:\rust-learning`:

```powershell
cd tasks/step_1_core_foundations/06_ownership_and_memory/project_06_borrowed_report
cargo check
cargo run
```

From this task folder, compare with the separate reference only after trying the task:

```powershell
cargo run --example solution
```

The reference does not overwrite your starter. A successful check only proves that code compiles. Compare results and explain your code too.

## Expected stdout

Captured from the reference example. Cargo's compile messages are stderr and are not part of this output. Passing assertions print nothing.

```text
=== STUDY TRACKER ===
Learner: Rahim
Daily goal: 90 min
Sessions:
1. Rust - 25 min (practice)
2. Rust - 30 min (practice)
3. Review - 20 min (review)
Total: 75 min
Progress: Below goal
Remaining: 15 min
Longest subject: Review
Sessions still available: 3
====================
```

## Hints — open one at a time

<details>
<summary>Hint 1: plan the data flow</summary>

Start with no selected subject. Visit each session through the slice. Remember a borrowed subject plus its scalar count. Change the selection only for a strictly larger count, or when there is no selection.

</details>

<details>
<summary>Hint 2: connect the Rust syntax</summary>

A partial starting declaration is `let mut longest: Option<&str> = None;`. Obtain text with `session.subject.as_str()`. `longest.is_none()` handles even a first empty subject. Return `longest` after the loop; match it in `main`.

</details>

## Acceptance check

No clone is used for reporting. Both helpers accept slices. Empty data never requires indexing or unwrap. Unicode and first-on-tie assertions pass, and the original list is printed as still available. Compare the exact stdout above. Explain one design choice aloud before considering this mission finished. Maintenance of this package does not mark your learning progress complete.

After your attempt, cross-check [the reference code](solution/solution.rs) and [its explanation](solution/explain.md).
