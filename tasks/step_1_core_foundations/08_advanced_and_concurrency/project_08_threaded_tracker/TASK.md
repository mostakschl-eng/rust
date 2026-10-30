# Checkpoint 8: Complete Your Study Tracker with a Worker Report

## Prerequisites and the continuing mission

Finish Lessons 01–80 and [checkpoint 7](../../07_errors_and_io/project_07_saved_tracker/TASK.md) first. This is the same Study Tracker, not an unrelated app. Keep Rahim's profile, the 90-minute goal, session lists and kinds, total/remaining minutes, goal status, and the borrowed longest-subject query. Longest means the most Unicode scalar values, with the first name winning a tie.

Your new mission: Keep checkpoint 7's commands, validation, session model, and saved-file format. Give one worker an owned snapshot, send its report through a channel, and join the worker. Only the main thread prints the report.

Each checkpoint is a standalone snapshot. Bring forward **your own** previous implementation into this TODO starter, then add the new mission. Do not import a later solution into an earlier stage. [The testing guide](../../../../guides/testing_basics.md) explains the assertion checks used here.

## Requirements

1. Use `Session { subject: String, minutes: u32, kind: StudyKind }` and `StudyKind::{Practice, Review}`.
2. Support `add <subject> <minutes> <practice|review>`, `list`, `report`, `help`, and `quit`. Ignore blank input; reject unknown commands and extra arguments. End-of-input must also exit.
3. A subject is one nonempty word. Minutes are a whole number from 1 to 1440. Reject bad input without adding a record.
4. Store `study_sessions.tsv` in the current working directory. Each line is subject, minutes, and lowercase kind separated by tabs, followed by a newline.
5. Load and validate all saved lines before accepting commands. Only a missing file means an empty tracker. Other read errors, invalid UTF-8, invalid records, or an unfinished last line must stop the program without replacing the file.
6. Append validated records; do not truncate the log. Stop on a write failure. Add a session to memory only after its append succeeds.
7. Use `u64` for total minutes. Return zero remaining minutes when the goal is reached or exceeded. An empty list must give total 0 and no longest name.
8. Compute reports on a worker using an owned snapshot, `thread::spawn`, and `mpsc`. Receive the result, join the worker, and handle worker/channel errors. Keep file operations and rendering on the main thread.

## Sample commands

Use a fresh empty working directory for this demonstration:

```text
add Rust 25 practice
add Rust 30 practice
add Review 20 review
list
report
quit
```

## Expected output

This is program stdout for those commands in a fresh directory. A terminal may echo what you type; Cargo messages and diagnostics are separate.

```text
=== STUDY TRACKER ===
Learner: Rahim
Daily goal: 90 minutes
Loaded sessions: 0
Commands: add <subject> <minutes> <practice|review> | list | report | help | quit
Saved: Rust (25 minutes, practice)
Saved: Rust (30 minutes, practice)
Saved: Review (20 minutes, review)
1. Rust: 25 minutes (practice)
2. Rust: 30 minutes (practice)
3. Review: 20 minutes (review)
Report source: worker thread
Sessions: 3
Total minutes: 75
Goal status: below goal
Remaining minutes: 15
Longest subject: Review
Goodbye.
```

On a second run in the same directory, `Loaded sessions: 3` should appear. Running only `report` and `quit` must still show 75 minutes and must not append the same sessions again.

## Run your version

From the repository root:

```text
cd tasks/step_1_core_foundations/08_advanced_and_concurrency/project_08_threaded_tracker
cargo check
cargo run
```

Try your own code first. [Reference code](solution/solution.rs) and [the explanation](solution/explain.md) are separate. Use `cargo run --example solution` only when ready to compare. Both versions use the current directory's log; use the disposable-folder instructions in the explanation to keep reference runs away from your own history.

## Hints

<details>
<summary>Hint 1: Keep old behaviour, change one boundary</summary>

Keep checkpoint 7's storage and commands unchanged. A worker needs data that stays valid independently of the command loop; clone a small snapshot for this teaching project, not the original live list's borrowed references.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Make the report computation return a String instead of printing. Move the snapshot and sender into the worker. Receive its String, join the handle, then print on the main thread; output order must not depend on thread timing.

</details>

## Acceptance checks

- Reproduce the sample, then restart and check that the three records remain.
- Try `add Rust 0 practice`, a negative or oversized number, an unknown kind, and extra arguments. None may change the log.
- Try an empty list, exactly 90 minutes, and more than 90; remaining minutes must never underflow.
- In a separate disposable directory, provide a malformed log and confirm a failed startup leaves it unchanged.
- End the input stream without typing quit. The app must exit.
- A log produced by checkpoint 7 must load unchanged here. Moving to the next folder does not move the file automatically: copy your own log only if the destination does not already have one; never overwrite another history.

This is a small **single-process learning app**. Do not run two writers against one log. An interrupted write can leave a partial final line; the app rejects that file on the next start so you can inspect a copy. Append-only storage is not a database transaction or a guarantee against power loss.

Next: [Step 2: choose libraries when needed](../../../../lessons/step_2_universal_crates/README.md).
