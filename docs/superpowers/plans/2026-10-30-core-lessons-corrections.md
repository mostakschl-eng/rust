# Step 1: correctness and beginner practice corrections

The user authorized correcting the existing 80 lessons across eight levels. Keep all folder names, the lesson order, separate reference solutions, and the student's progress. Do not run Git commands.

1. Read each lesson and its matching task, starter, reference solution, and explanation. Correct technical claims and examples. Include analogies, symbol tables, expected output, and pitfalls.
2. Keep complete solutions in `solution/`. Save copies of completed starter files outside the repository before replacing them with unfinished, task-specific TODO scaffolds. Task 20 is intentionally broken for compiler-diagnostic practice.
3. Supply progressive hints, correct expected output, and make Task 67 genuinely read terminal input. Preserve the supplied small exercise scope; do not create the planned later stages.
4. Repair Markdown navigation and shell paths after content edits. Clarify installation, the beginner workflow, and which later stages are outlines.
5. Verify all package starters with `cargo check` (expect the documented errors for Task 20). Verify all solutions with `cargo run`, including real terminal input, output comparisons, and targeted edge-case assertions. Check all local links and required files.

Verification must use `cargo check` or `cargo run`, as required by `AGENTS.md`. Temporary manifests, run output, and starter backups belong in the writable visualization directory, not in exercise folders. Do not check off lessons or alter the student's current learning position.

## Completed verification

- Reviewed all 80 lesson/task/solution bundles. Preserved separate solutions, folder names, and existing partial attempts in Tasks 26 and 28.
- All 80 reference solutions ran successfully through their real manifest's `solution` example target. All 80 documented program outputs matched, with Task 19's variable stderr diagnostic recorded separately.
- All 79 ordinary starters passed `cargo check`. Task 20 produced the intended compiler errors; the additional mutability error was also verified after isolating it from the other intentional errors.
- Checked 178 complete lesson code blocks: 169 compiled, including the documented multi-file example with its companion module; nine deliberately invalid teaching examples produced compiler errors. Verified focused numeric, Unicode, parser, account, card-slicing, and XP edge cases through `cargo run`.
- All 80 lessons contain code, expected-output blocks, symbol tables, and pitfalls. All 80 tasks contain their required files and at least two hidden hints. No starter matches its complete reference solution after removing comments and blank lines.
- All 503 local Markdown links in the maintained learning material resolve. Preserved learning progress; only its link targets were made portable.
- Saved 54 original starter files outside the repository before replacement or targeted correction. An independent reader reviewed ownership and concurrency corrections; all six remaining findings were addressed and confirmed resolved.

Verification used the installed Cargo 1.98.0 on Windows. Older toolchains and other operating systems were not run. Steps 2–4 remain outlines. No Git commands were run.
