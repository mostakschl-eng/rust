# Study Tracker — Checkpoint 5: Give sessions a clear shape

## Before you start

Finish Lessons 01–50 and their numbered tasks first. Review [the relevant lesson](../../../../lessons/step_1_core_foundations/05_structs_and_enums/44_struct_methods.md) and the [checking guide](../../../../guides/testing_basics.md).

[Previous checkpoint](../../04_collections/project_04_session_collection/TASK.md) · [Next checkpoint](../../06_ownership_and_memory/project_06_borrowed_report/TASK.md)

This is one growing Study Tracker. Each folder is a separate, runnable snapshot. Repeated code is intentional: do not import a later checkpoint into an earlier one. Rebuild the retained features here so this folder works alone.

## What stays

Keep the learner profile, daily goal, session list, total study minutes, remaining minutes, and goal-progress decision. These are connected missions, not unrelated apps.

## Your new mission

Replace the tuples with named Session records. Add a fixed StudyKind choice. The tracker still describes the same study day.

`&self` was introduced in Lesson 44: a method reads a session without taking it away. `&Vec<Session>` applies that same read-only borrowing idea to the list. `match &self.kind` inspects the enum through a reference. Level 6 explains these ownership rules more deeply. `format!` uses the same `{}` placeholders as `println!`, but returns a `String` instead of writing a line.

## Requirements

1. Keep Rahim's profile, the 90-minute goal, numbered list, total, progress, remaining minutes, header, and footer.
2. Define `enum StudyKind { Practice, Review }`.
3. Define `struct Session` with `subject: String`, `minutes: u32`, and `kind: StudyKind`.
4. Add `Session::new(subject: String, minutes: u32, kind: StudyKind) -> Self`. Add `description(&self) -> String`. Match both variants explicitly to produce lowercase kind names. Use `format!` to return the complete row description without printing it.
5. Build Rust/25/practice, Rust/30/practice, Review/20/review, in order. Subjects are owned strings made with `String::from`.
6. Write `total_minutes(sessions: &Vec<Session>) -> u64` using a loop. Retain `remaining_minutes` and `progress_status` from checkpoint 4.
7. Check the canonical total, empty vector, both kind descriptions, and totals below/equal/above the goal with silent assertions.
8. Keep this a fixed-data program. No keyboard input, file storage, custom traits, or iterator pipelines.

This is a small fixed-data exercise. The supplied records are bounded. A `u64` accumulator avoids adding the sample's `u32` minute values in a narrower type; it is not a promise of unlimited totals.

## Work and run

Edit [src/main.rs](src/main.rs). It deliberately compiles before the work is finished; its current output is not the target output.

Starting at the repository root `D:\rust-learning`:

```powershell
cd tasks/step_1_core_foundations/05_structs_and_enums/project_05_session_models
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
====================
```

## Hints — open one at a time

<details>
<summary>Hint 1: plan the data flow</summary>

A struct gives each piece of the old tuple a name. A constructor builds one record. A method can describe that record without consuming it.

</details>

<details>
<summary>Hint 2: connect the Rust syntax</summary>

Inside `description`, match `&self.kind` with one arm for `StudyKind::Practice` and one for `StudyKind::Review`. Store the resulting label, then use `format!` with the subject, minutes, and label.

</details>

## Acceptance check

No tuple positions remain in the session model. Both variants are described through a match. The three reference rows retain their original subjects and minutes. Assertions cover empty data and the three goal states. Compare the exact stdout above. Explain one design choice aloud before considering this mission finished. Maintenance of this package does not mark your learning progress complete.

After your attempt, cross-check [the reference code](solution/solution.rs) and [its explanation](solution/explain.md).
