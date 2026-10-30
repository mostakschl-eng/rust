# One Study Tracker, eight checkpoints

The user authorized eight projects and then asked for connected missions. Build one growing Study Tracker with eight independently runnable checkpoint packages. Keep all existing 80 lessons, task answers, student attempts, and progress states. Do not run Git commands.

Each checkpoint lives in its matching existing level folder under `tasks/step_1_core_foundations/`:

1. `project_01_profile_card`: printed learner profile, goal, reusable header/footer; only Lessons 01–10.
2. `project_02_study_minutes`: retain the profile; calculate 3 sessions × 25 minutes = 75, daily goal 90, remaining 15. Introduce assertion checks through a linked guide; no keyboard input or loops.
3. `project_03_goal_tracker`: retain profile/minute calculation; loop over 3 fixed sessions and classify progress against the goal. Check below/equal/above goal; no randomness or keyboard input.
4. `project_04_session_collection`: represent the same three sessions as a vector of tuples: Rust25, Rust30, Review20. Print sessions, total75, goal90, remaining15; demonstrate empty collections. No structs or file I/O.
5. `project_05_session_models`: replace tuples with `Session { subject: String, minutes: u32, kind: StudyKind }` and `StudyKind::{Practice, Review}`. Keep the same data and report. Add methods/enum matching; no user input or persistence.
6. `project_06_borrowed_report`: keep the model and features; compute reports using borrowed slices, return a borrowed longest-subject name with a lifetime, handle empty slices without panicking. No clones required for read-only reports. No user input or persistence.
7. `project_07_saved_tracker`: keep profile, goal, session model, list and totals; add a terminal command loop and an append-only TSV log. Commands: `add <subject> <minutes> <practice|review>`, `list`, `report`, `help`, `quit`. Subjects are one nonempty word; minutes1..=1440; goal90. Load and validate saved records before accepting commands; report malformed files without overwriting them. Missing file starts empty. Persist an addition successfully before adding it to memory. No external crates.
8. `project_08_threaded_tracker`: retain Level7 app, command/data format and persistence; replace report computation with a worker using `thread::spawn`, owned snapshot, and `mpsc`. Main receives and joins worker; rendering order remains deterministic. Iterators may summarize borrowed data. Keep a clearly stated small local-demo scope, not a server.

Each folder must have Cargo.toml, TASK.md, compilable unfinished src/main.rs, complete solution/solution.rs and detailed solution/explain.md. Expose `solution` as an example target, use no dependencies, provide two hidden hints, exact captured sample output, prerequisites, previous/next checkpoint links, and acceptance criteria. Explicitly explain intentional repeated snapshots: later code must not be imported into an earlier checkpoint.

Root documentation must keep exactly four top-level steps: foundations with small projects/checking, selected libraries, one specialization, then larger projects in that specialization. Tokio and UI/web libraries are optional according to need. Introduce assertion checking after Lesson20 and formal unit-test syntax after modules; quality practice starts before Step4. Steps2–4 stay clearly labelled outlines.

Verify all eight starters with cargo check; run all eight reference examples, including interactive success, restart/reload, malformed-file, invalid-input, EOF and persistence-failure cases. Use disposable run directories. Compile the educational test examples with cargo check and exercise equivalent assertions through cargo run; do not run cargo test under the existing agent rules. Check local links and learning-progress preservation. Request independent review of persistence, sequencing, and the final checkpoint.

## Completed checks

- Created eight standalone checkpoint packages with required task/starter/solution/explanation files, no dependencies, and separate `solution` example targets. All eight starters pass `cargo check`; all eight solutions run and match captured sample stdout.
- Runtime assertions cover arithmetic, goal boundaries, empty collections, enum descriptions, borrowed Unicode names and ties, parsing, and worker/direct-report agreement. Disposable mutations in checkpoints 2–3 demonstrate that the checks catch incorrect arithmetic and classification.
- Ran 19 integration scenarios across checkpoints 7–8: reloading without duplicate writes, invalid commands/ranges/kinds, empty and exact/above-goal reports, EOF, malformed/unfinished logs, invalid UTF-8, non-missing read errors, and checkpoint 7 history read by checkpoint 8.
- Two additional Windows sharing-lock scenarios reached the append phase, rejected the write, printed no false `Saved` message, exited with an error, and preserved the existing log bytes. Total integration scenarios: 21.
- An independent reader reviewed checkpoints 7–8 and their progression from checkpoint 6; no blocking or important findings remained.
- The testing guide's two complete examples pass `cargo check` and `cargo run`; formal test code also passes `cargo check --tests`. The formal test runner was not executed under the repository's agent-command rules.
- Added navigation at the end of each ten-lesson level. All 601 local links checked in the maintained root/Step 1/guide material resolve. The four-step roadmap and later-stage outlines were clarified; all 80 lesson and eight milestone completion boxes remain unchecked.
- Existing student source files and learning-state values remain unchanged. Only new checkpoint packages, documentation, and tutor workflow instructions were added or updated. No Git commands were run.

Verification used the installed Windows Rust toolchain. These are small single-process educational applications; other operating systems, older Rust versions, multi-process writers, and power-loss recovery were not validated.
