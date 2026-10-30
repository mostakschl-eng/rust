# Solution Explanation: Task 62 (Option Enum)

Let's dissect how returning `Option<T>` clearly communicates that an operation may yield a missing value!

---

## 🔍 The Solution Code

```rust
fn find_player_score(player_id: u32) -> Option<u32> {
    match player_id {
        101 => Some(950),
        102 => Some(820),
        _ => None,
    }
}
```

---

## 🧩 Key Takeaways

1. **Clear Function Signature:**
   * Anyone calling `find_player_score` knows from the return type `Option<u32>` that the player might not be registered.
2. **`.is_some()` vs `match`:**
   * `.is_some()` is convenient for quick boolean checks.
   * `match` unpacks the inner value cleanly.


## Reading the Result Carefully

A lookup desk returns either a record envelope or an explicit no-record slip. A zero score is a record, not a missing record.

| Symbol or term | Meaning |
|---|---|
| `Some(0)` | present zero |
| `None` | no value |
| `.is_some()` | boolean presence check |

Testing is_some does not extract the number. Use match to bind the inner value; an unknown player should not become a zero score.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
