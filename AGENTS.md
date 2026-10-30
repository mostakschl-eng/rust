# Agent Rules & Guidelines (`AGENTS.md`)

## ⚠️ STRICT RULE: DO NOT TOUCH GIT
Any AI agent working in this repository must strictly follow these rules:
1. **NEVER run `git add`, `git commit`, `git push`, or any staging/commit commands.**
2. The student manages all Git commits manually.
3. Only create, edit, or read files, and verify tasks using `cargo check` or `cargo run`.
4. Do NOT use the `context-mode` MCP plugin.
5. All lessons must be written in simple, deep English with real-life analogies, symbol tables, code examples, expected outputs, and common pitfalls.
6. Every task folder must have:
   - `TASK.md` (instructions, expected output, hints)
   - `src/main.rs` (starter code with `// TODO`)
   - `solution/solution.rs` (verified solution)
   - `solution/explain.md` (detailed explanation)

## Required Accuracy Rules for Every Agent

These rules apply when creating, reviewing, or changing any lesson, task, example, hint, solution, or explanation. Correctness comes before adding more content. A previous AI answer or an existing lesson is not proof that a claim is correct.

### 1. Read Before Writing

- Read `TUTOR.md` and the relevant part of `ROADMAP.md` first.
- Read the entire affected bundle: the lesson, `TASK.md`, `src/main.rs`, `solution/solution.rs`, `solution/explain.md`, and `Cargo.toml`.
- Check earlier lessons for prerequisites. Explain unfamiliar syntax briefly before using it; do not quietly require concepts taught much later.
- Keep the existing lesson order and folder structure unless the student explicitly requests a change. Fix the same mistake wherever it appears in the affected bundle.

### 2. Establish the Rust Facts

- Verify technical claims against the official Rust Book, standard-library documentation, Rust Reference, or Cargo Book. Read the actual supporting section when adding or changing an uncertain claim; do not treat search snippets or AI-generated summaries as evidence.
- Prefer these primary references: [Rust Book](https://doc.rust-lang.org/book/), [standard library](https://doc.rust-lang.org/std/), [Rust Reference](https://doc.rust-lang.org/reference/), and [Cargo Book](https://doc.rust-lang.org/cargo/).
- Use a small `cargo check` or `cargo run` experiment when behaviour can be demonstrated. A successful compile proves neither the expected result nor the explanation; check both separately.
- Distinguish a language guarantee from behaviour observed on one compiler or machine. State platform, compiler, alignment, optimization, and scheduling limits where they matter. Do not turn one measured size or run into a universal rule.
- If evidence is missing or conflicting, investigate before adding the claim. If verification cannot be completed, report exactly what remains uncertain instead of presenting a guess as fact.

### 3. Verify Examples and Expected Output

- Check every new or changed complete code example. Supply every required definition, import, and companion file. Label partial snippets as partial and say which earlier definitions they need. Do not put `...` inside a supposedly runnable program.
- Run each affected reference solution using `cargo run --example solution` from its task folder. Keep the `solution` example target in `Cargo.toml`; do not replace the student's code to run an answer.
- Capture expected output from the actual run. Never calculate byte counts, word counts, numeric results, or rounding by intuition alone.
- Separate program stdout from compiler messages, warnings, and stderr such as `dbg!`. Document variable file locations, OS error messages, terminal input, and thread ordering. Do not promise one exact ordering when it is not guaranteed.
- For a deliberately invalid example, confirm that it fails for the stated reason. If the invalid line is commented out, explicitly say to uncomment it. Never invent compiler diagnostics; label diagnostic excerpts as representative when wording varies.
- Ordinary unfinished starters must pass `cargo check`; they do not have to produce the completed output. Task 20 deliberately fails until its three bugs are repaired. Do not fix that exercise into a completed starter.
- Exercise the meaningful boundaries of changed logic with small assertions or focused runs: empty input, invalid input, boundaries, overflow, Unicode, and failure paths where relevant. Use `cargo check` and `cargo run`, as required above; do not introduce another verification command to bypass this rule.
- Run file-writing examples in a disposable directory. Handle write failures and avoid overwriting student files. Check real stdin for an input lesson, not only a hardcoded string.

### 4. Avoid Misleading Simplifications

- An analogy helps explain a rule; it is not the rule itself. State where the analogy stops being accurate. Use simple English without replacing accurate terms with false claims.
- Do not say Rust guarantees zero crashes, zero leaks, correct business logic, or freedom from every concurrency bug. Explain the particular guarantee and its limits.
- Moving is a default value behaviour, not a standard `Move` trait. `Copy` is not determined by stack versus heap storage. `Clone` does not universally mean a deep copy or a new allocation.
- Explain bytes, Unicode scalar values (`char`), and visible characters separately. String slicing uses byte positions and must respect UTF-8 boundaries; `.chars().count()` is not a general visible-character count.
- Do not promise exact enum sizes by adding field sizes. Distinguish a fixed-size handle from its payload, include alignment/layout limits, and avoid claiming every value is physically stored on the stack.
- `move` controls closure capture; it does not by itself mean a closure can run only once. Explain overlapping closure traits, active borrows, lifetimes, and concurrency without absolute rules contradicted by valid Rust examples.
- Floating-point examples teach approximate arithmetic. Formatting to two decimals does not make values exact money. Mark simulated banking, payment, authentication, and security examples as demonstrations; do not claim they implement real services or protection.

### 5. Preserve Practice and Student Work

- Keep complete answers in `solution/`. Keep `src/main.rs` as a useful unfinished exercise with task-specific `// TODO` guidance; a TODO comment followed by its complete answer is not a starter.
- Put progressive help in at least two hidden hint sections in `TASK.md`. Let the learner choose when to reveal it. Do not expose the complete solution in the starter or ordinary instructions.
- Preserve identifiable student attempts, including unfinished or incorrect work. Before replacing a completed starter or making a destructive correction, save a clearly named copy outside the task folder and report its location. Do not reset the student's work merely to make checks pass.
- Never mark lessons complete or advance `TUTOR.md`, `ROADMAP.md`, or `progress/log.md` because maintenance was done. Verify actual student completion before changing learning status.

### 6. Final Quality Gate

Before reporting an affected lesson or task as corrected:

1. Confirm the lesson has a simple explanation, real-life analogy, symbol table, code examples, expected output, and common pitfalls.
2. Confirm all required task files exist, hints progress sensibly, starters remain unfinished, and the solution and explanation agree with the requirements.
3. Check every changed local Markdown link and terminal path. Use relative links. State the starting directory for `cd tasks/...` commands.
4. Re-read the explanation against the code and observed results. Check for contradictory summaries, comments, tables, and copied explanations. For ownership, lifetimes, Unicode, numeric conversion, or concurrency changes, review an edge case that would disprove an overly broad statement.
5. Run the relevant checks after the final code changes. Recheck dependent examples when a shared explanation, requirement, or function changes. Documentation-only edits need a content/link review; do not claim code was rerun if it was not.
6. Report what changed, the checks actually run, and any remaining limits. Do not claim compatibility with untested Rust versions or operating systems, and do not describe future-stage outlines as completed lessons.

Do not call unverified content "perfect," "fully correct," or "verified." No instruction file can guarantee that mistakes never happen; these evidence requirements are mandatory safeguards against repeating them.

