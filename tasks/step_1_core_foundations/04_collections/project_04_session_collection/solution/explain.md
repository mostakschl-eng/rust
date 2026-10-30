# Checkpoint 4 — reference explanation

Read this after trying [the task](../TASK.md). The [complete runnable code](solution.rs) is the example target `solution`. The fragments below are partial excerpts from that program; they are not separate programs.

## A notebook with one line per session

Earlier checkpoints knew how many sessions to repeat. Now the list itself records the work. A tuple is like one notebook line with two columns: subject and minutes. A vector is the notebook's list of lines. Unlike a paper notebook, this list has one element type checked by Rust: every element here is a `(&str, u32)` tuple.

The model does not yet record practice versus review as a separate field. That arrives in checkpoint 5. For now, Review is the subject name of the third record.

## Read the data and loop

Partial excerpt from `main`:

```rust
let sessions: Vec<(&str, u32)> = vec![("Rust", 25), ("Rust", 30), ("Review", 20)];
```

`Vec<...>` names the element type. `vec![...]` builds the actual vector. The strings here are fixed borrowed text literals. This chapter does not make a growable subject string.

Partial excerpt; requires the `sessions` definition above:

```rust
for &(subject, minutes) in &sessions {
    println!("{} - {} min", subject, minutes);
}
```

The outside `&sessions` lends the list for reading. Each loop item is a reference to a tuple. The pattern `&(subject, minutes)` unpacks that reference and copies its small `&str` and integer values into local names. It does not copy the subject's text. Copying these values is not decided by whether something is on the stack or heap.

The full program adds a separate counter starting at one. The counter belongs to display, not to the stored record. The list order supplies the row order.

A shared vector reference is a small bridge from Lesson 37 to reusable helpers. It keeps the caller's vector available. Checkpoint 6 will widen that parameter from a vector reference to a slice.

## Calculate instead of hardcoding

Partial excerpt from `total_minutes`; requires its `sessions: &Vec<(&str, u32)>` parameter:

```rust
let mut total = 0_u64;
for &(_subject, minutes) in sessions {
    total += minutes as u64;
}
total
```

The subject is not needed for addition. The leading underscore in `_subject` communicates that. `0_u64` fixes the accumulator type. Conversion happens before addition. A `u32` value fits in `u64`, so this widening conversion loses no minute value.

The accumulator starts at zero. With an empty vector, the loop body never runs, so zero is returned. The final `total` has no semicolon because it is the function's returned expression.

This uses a small, fixed study day. Wider integers have limits too. The program does not claim to handle an unlimited list or arbitrary total.

## Retain the old goal decision

The total helper knows nothing about the daily goal. The remaining helper subtracts only when the total is below the goal. At or above the goal, it returns zero. This avoids unsigned subtraction going below zero.

The status helper separately returns a `String` for the three cases. Equal and above are deliberately different. A return type of `String` reuses familiar text syntax without requiring a written lifetime annotation at this checkpoint.

The silent assertions check the sample list, empty list, and totals below, equal to, and above 90. A passing assertion prints nothing. A failing assertion stops the run. These checks exercise chosen cases; they do not prove all possible inputs.

## Symbol table

| Syntax | Read it as |
|---|---|
| `Vec<(&str, u32)>` | A vector of subject/minute pairs |
| `vec![...]` | Build a vector from the listed values |
| `Vec::new()` | Make an empty vector |
| `&sessions` | Lend shared access to this list |
| `&(subject, minutes)` | Unpack a referenced pair of copyable values |
| `+=` | Add to the existing accumulator |
| `as u64` | Convert this minute value to a wider integer |
| `assert_eq!(a, b)` | Stop if the two values differ |

## Observed stdout

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

Run `cargo run --example solution` from this task folder. Compiler messages are separate from this stdout.

## Common pitfalls

- Printing three fixed lines hides whether the loop works. Build rows from the collection.
- Forgetting `&` in a vector loop can consume the vector. Borrow when you need to reuse it.
- An empty vector needs an element type when nothing else establishes it.
- Do not subtract first and check later. Unsigned arithmetic cannot represent negative remaining minutes.
- A tuple is compact, but its field meanings depend on position. The next checkpoint gives those fields names.
- This is a snapshot of the growing tracker. Importing checkpoint 8 would hide the very skills being practised.

The official [Rust Book section on vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html) supports the vector creation and shared-loop behavior used here.
