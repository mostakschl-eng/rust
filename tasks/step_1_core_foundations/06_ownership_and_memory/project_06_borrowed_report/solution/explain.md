# Checkpoint 6 — reference explanation

Read this after trying [the task](../TASK.md). The [complete runnable code](solution.rs) is the example target `solution`. The fragments below are partial excerpts from that program; they are not separate programs.

## Lend the notebook, keep the notebook

Imagine lending someone a view of your study notebook. They read its lines and point to the longest subject already written there. They do not tear out the page or write a duplicate name.

A borrowed slice is the read-only view. The returned `&str` points into a session's existing subject. The analogy stops at ownership rules: Rust checks whether that borrow can still be used safely. A real notebook has no compiler.

The Session/StudyKind model stays the same. The report changes how it receives data.

## A slice accepts a whole list or a window

`&[Session]` is a shared slice of sessions. It does not own them. `&sessions` can be converted to such a slice when passed to the helper. A borrowed array or a valid part of a vector can also be used.

Partial excerpt from `main`; requires the canonical `sessions` vector and `total_minutes` from `solution.rs`:

```rust
assert_eq!(total_minutes(&sessions[0..2]), 55);
```

The range includes indices zero and one, not two. This particular range is valid because the known sample contains three records. Arbitrary out-of-bounds ranges can panic; slices are not a promise that every range is valid.

The total helper reads each record, widens its `u32` minutes to `u64`, and adds them. An empty slice runs the loop zero times and returns zero. This remains a small fixed-data report with bounded examples.

## Pick a borrowed subject

The function starts with `None`: it has not selected a subject. For each record it obtains a borrowed `&str` through `as_str()`, counts scalar values, and updates the selection only when needed.

Partial excerpt from `longest_subject`; requires the loop's `session` plus `longest: Option<&str>` and `longest_count` defined in `solution.rs`:

```rust
let subject = session.subject.as_str();
let count = subject.chars().count();
if longest.is_none() || count > longest_count {
    longest = Some(subject);
    longest_count = count;
}
```

`is_none()` asks whether there is no selected subject. It matters for a first subject with zero scalar values. Without that condition, an empty subject could be confused with an empty list. `Some("")` means a record exists with empty text. `None` means no record exists.

Using strict `>` preserves the first subject when two counts are equal. The function never indexes the first item and never unwraps an absent result.

## What does longest mean for Unicode?

This exercise deliberately counts Unicode scalar values with `.chars().count()`. The specific operation is introduced here for text comparison; general iterator pipelines come in Lesson 77.

Byte length and scalar count are different. `éé` uses more UTF-8 bytes than `abc`, but has fewer scalar values. The assertion chooses `abc`, then keeps it when `xyz` has the same scalar count. Replacing the comparison with `.len()` would break that check.

Scalar values are not always visible characters. A base letter plus a combining accent can contain multiple scalar values while appearing as one character. This task does not count grapheme clusters, normalize Unicode, or slice strings by character position. The standard-library [str::chars documentation](https://doc.rust-lang.org/std/primitive.str.html#method.chars) explains this distinction.

## The lifetime exists even when its name is omitted

The signature is `fn longest_subject(sessions: &[Session]) -> Option<&str>`. It has one input lifetime. Elision connects that input borrow to the output borrow automatically. An explicit `'a` is not required.

To name this relationship, `<'a>` declares a lifetime parameter, `&'a [Session]` marks the input borrow, and `Option<&'a str>` marks the output borrow. These annotations would describe the same relationship; they would not keep the vector alive longer. The [Rust Book's lifetime elision section](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision) explains the one-input rule.

A subject borrowed from the vector cannot be used after its source storage is dropped. An edit requiring mutable access must also respect any overlapping shared borrow still used later. The borrow may end after its last use; it does not automatically last to the end of the enclosing block. Printing the vector's length afterward uses shared access and remains valid.

The model owns its strings; it does not store borrowed fields. No custom lifetime parameter is needed on Session itself.

## Retained features and focused checks

The constructor, kind match, numbered list, goal, status, and remaining minutes still work. Only report parameters and the extra longest-subject result are new.

The example checks the canonical longest name, no records, an empty name, Unicode, first-on-tie, a sub-slice, total, and goal boundaries. At the end it reads the original vector again. Borrowing has not consumed it.

The longest-subject helper makes no new subject string. The existing `description` method still uses `format!`, which creates an output string. Read-only reporting requires no clones here; it does not mean every part of formatting is allocation-free.

## Symbol table

| Syntax | Read it as |
|---|---|
| `&[Session]` | A borrowed view of session records |
| `&sessions[0..2]` | Borrow the first two records |
| `as_str()` | View an owned String as borrowed text |
| `Option<&str>` | A borrowed name, or no name |
| `Some(subject)` | A selected subject exists |
| `None` | No subject was selected |
| `is_none()` | Ask whether the selection is absent |
| `.chars().count()` | Count Unicode scalar values |
| `'a` | A name for a borrowing relationship |
| `>` | Strictly larger; equal counts retain the first |

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
Longest subject: Review
Sessions still available: 3
====================
```

Run `cargo run --example solution` from this task folder. Checks are silent on success.

## Common pitfalls

- Returning a borrow of a new local String would refer to data dropped when the function returns. Return a view into the supplied records.
- Do not return an owned clone just to avoid understanding the borrowed result.
- A lifetime annotation does not extend storage life.
- A slice permits empty data. Handle that case before any assumption that a record exists.
- Changing `>` to `>=` makes the last equal-length subject win.
- Bytes, scalar values, and visible characters are three different counting ideas.
- Keeping the vector available is compatible with shared borrowing. It does not authorize mutation while an overlapping shared borrow remains in use.

The official [Rust Book section on slices](https://doc.rust-lang.org/book/ch04-03-slices.html) supplies the borrowed-view model.
