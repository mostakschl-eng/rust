# Checkpoint 5 — reference explanation

Read this after trying [the task](../TASK.md). The [complete runnable code](solution.rs) is the example target `solution`. The fragments below are partial excerpts from that program; they are not separate programs.

## Turn notebook columns into a form

A tuple is a short row whose meaning depends on order. A `Session` is like a printed form with named boxes: subject, minutes, and kind. The enum is like a form with two permitted kind choices. Unlike a paper form, Rust rejects a variant name that the enum does not define.

This model organizes data. It does not validate real study activity or prevent code in this same module from changing a field directly.

## Follow one session from construction to display

The enum names two alternatives: `StudyKind::Practice` and `StudyKind::Review`. A session contains one of them. It also owns its subject as a `String`.

Partial excerpt from `main`; requires the `Session` and `StudyKind` definitions and `impl Session` in `solution.rs`:

```rust
let session = Session::new(String::from("Rust"), 25, StudyKind::Practice);
println!("{}", session.description());
```

This excerpt prints `Rust - 25 min (practice)`. It is the same first record used in the captured full output below.

`new` is an associated function. Call it through the type name because no session exists yet. Its `Self` return type means `Session` inside this implementation block. The constructor uses field shorthand: `Self { subject, minutes, kind }` fills each named field from its same-named parameter.

`description` is a method. Call it through one session. Its `&self` receiver borrows that session for reading, so describing it does not consume it. The constructor receives the owned values; the description only inspects them.

## Match every allowed kind

Partial excerpt from `description`; requires its enclosing `impl Session` and `&self` parameter:

```rust
let kind_name = match &self.kind {
    StudyKind::Practice => "practice",
    StudyKind::Review => "review",
};
format!("{} - {} min ({})", self.subject, self.minutes, kind_name)
```

Matching `&self.kind` reads the kind through a borrow. The arms produce fixed labels. The match handles both variants explicitly. If the enum gains another variant, this match must handle it too.

`format!` returns a newly formatted `String`. It does not print. The final expression returns that string to the caller, which can print or compare it. `println!` in the outer loop controls the numbered report.

No clone is needed to inspect the record. The formatted description is still a new string. Borrowing an input does not mean every operation on it makes no allocation.

## Keep the earlier tracker working

The collection is now `Vec<Session>`. The read helper accepts `&Vec<Session>`, continuing Lesson 44's borrowing idea for the entire list. Within its loop, `session.minutes` reads a named numeric field instead of unpacking tuple positions.

Each `u32` minute count is converted to `u64` before accumulation. This fixed sample is small; using `u64` does not make integer overflow impossible for arbitrary data.

The same helpers still classify total against goal and clamp remaining minutes to zero at or above the goal. Structs have changed the representation, not the meaning of the study day.

## Why the checks matter

The reference checks empty data, the 75-minute sum, both kind descriptions, and below/equal/above goal. Checking both descriptions would catch a match that mistakenly labels review as practice. Checking equality catches a decision that sends the exact goal into the wrong branch.

Assertions are ordinary code executed by this example. Success is silent. They are a small check of the required behavior, not a complete test framework.

## Symbol table

| Syntax | Read it as |
|---|---|
| `struct Session` | A named record shape |
| `subject: String` | The subject field owns text |
| `enum StudyKind` | One choice from the listed variants |
| `StudyKind::Review` | The Review alternative |
| `impl Session` | Functions associated with this type |
| `Self` | This implementation's type, Session |
| `Session::new(...)` | Build a new session |
| `&self` | Borrow this session to inspect it |
| `=>` | Connect a match pattern to its result |
| `format!(...)` | Produce a formatted String |

## Observed stdout

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

Run `cargo run --example solution` from this task folder. Passing assertions add no lines.

## Common pitfalls

- A constructor does not need `&self`: there is no existing record to inspect.
- Returning a description and printing it are different jobs. Avoid printing inside `description`.
- Do not write just one catch-all match arm. The task practises the two meaningful choices.
- Do not replace `String` with a stored borrowed subject here. The model deliberately owns each subject.
- Keep the starter unfinished until you solve its TODOs. Checking the reference does not establish student completion.
- No command loop or file storage belongs in this checkpoint.

See the official [Rust Book section on methods and associated functions](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) for receiver borrowing and constructor syntax.
