# Study Tracker — Checkpoint 4: A real session collection

## Before you start

Finish Lessons 01–40 and their numbered tasks first. Review [the relevant lesson](../../../../lessons/step_1_core_foundations/04_collections/37_vector_iteration.md) and the [checking guide](../../../../guides/testing_basics.md).

[Previous checkpoint](../../03_logic_and_loops/project_03_goal_tracker/TASK.md) · [Next checkpoint](../../05_structs_and_enums/project_05_session_models/TASK.md)

This is one growing Study Tracker. Each folder is a separate, runnable snapshot. Repeated code is intentional: do not import a later checkpoint into an earlier one. Rebuild the retained features here so this folder works alone.

## What stays

Keep the learner profile, daily goal, session list, total study minutes, remaining minutes, and goal-progress decision. These are connected missions, not unrelated apps.

## Your new mission

Replace the fixed session repetition with a vector of tuples. One tuple is one study record. The vector is the day's growing list.

Lesson 37 already loops over `&prices`. Here `&sessions` also lets a helper read the existing vector. The helper receives a shared reference, not a new vector. We will study the ownership rules in Level 6. For now, keep the vector available by borrowing it.

## Requirements

1. Keep learner `Rahim`, a daily goal of `90` minutes, a header and footer.
2. Store exactly `("Rust", 25)`, `("Rust", 30)`, and `("Review", 20)` in a `Vec<(&str, u32)>`, in that order.
3. Use a borrowed `for` loop to print numbered sessions. Do not write three separate print statements.
4. Write `total_minutes(sessions: &Vec<(&str, u32)>) -> u64`. Start at zero and add each minute value after converting it to `u64`.
5. Restore `remaining_minutes(total: u64, goal: u64) -> u64` and `progress_status(total: u64, goal: u64) -> String`. Remaining minutes are zero at or above the goal. Status is `Below goal`, `Goal reached`, or `Above goal`.
6. Add silent assertions: the sample totals 75; an empty vector totals zero and leaves all 90 minutes; totals 75, 90, and 100 produce correct status and remaining values.
7. No structs, custom enums, keyboard input, files, or iterator pipelines. Use the collection and loops you have learned.

This is a small fixed-data exercise. The supplied records are bounded. A `u64` accumulator avoids adding the sample's `u32` minute values in a narrower type; it is not a promise of unlimited totals.

## Work and run

Edit [src/main.rs](src/main.rs). It deliberately compiles before the work is finished; its current output is not the target output.

Starting at the repository root `D:\rust-learning`:

```powershell
cd tasks/step_1_core_foundations/04_collections/project_04_session_collection
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
1. Rust - 25 min
2. Rust - 30 min
3. Review - 20 min
Total: 75 min
Progress: Below goal
Remaining: 15 min
====================
```

## Hints — open one at a time

<details>
<summary>Hint 1: plan the data flow</summary>

Each tuple groups a subject and its minutes. Read each tuple through `&sessions`. Keep the accumulator separate from the list.

</details>

<details>
<summary>Hint 2: connect the Rust syntax</summary>

The partial loop pattern `for &(subject, minutes) in &sessions` copies the tuple's `&str` and `u32` values for reading. In the total helper, use `minutes as u64` before adding. Check `total < goal` before subtracting.

</details>

## Acceptance check

The list comes from one vector, including its numbered rows. Empty-data and goal-boundary assertions pass. The starter remains separate from the reference. Compare the exact stdout above. Explain one design choice aloud before considering this mission finished. Maintenance of this package does not mark your learning progress complete.

After your attempt, cross-check [the reference code](solution/solution.rs) and [its explanation](solution/explain.md).
