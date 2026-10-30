# Solution Explanation: Task 57 (Golden Borrowing Rule)

Let's dissect how cleanly organizing reference lifetimes prevents borrow checker conflicts!

---

## 🔍 The Solution Code

```rust
fn main() {
    let mut audit_log = String::from("[Entry 1: System Boot]");

    // Phase 1: Multiple Readers (r1, r2)
    let r1 = &audit_log;
    let r2 = &audit_log;
    println!("Inspector A: {}", r1);
    println!("Inspector B: {}", r2);

    // Phase 2: Single Writer (writer)
    let writer = &mut audit_log;
    writer.push_str(" -> [Entry 2: Backup Verified]");
    println!("Lead Auditor updated log: {}", writer);

    // Phase 3: Final Reader (final_reader)
    let final_reader = &audit_log;
    println!("Final Log State: {}", final_reader);
}
```

---

## 🧩 Key Takeaways

1. **Why It Compiles:**
   * In Phase 1, `r1` and `r2` are shared references. They are printed and never referenced again.
   * Because of Non-Lexical Lifetimes, their borrow ends at line 7.
   * In Phase 2, `writer` can acquire exclusive mutable access safely.
   * `writer` is printed and never used again. Its mutable borrow ends at line 12.
   * In Phase 3, `final_reader` acquires a fresh read-only reference safely.
2. **Never Overlap Readers and Writers:**
   * If you tried to print `r1` AFTER `writer.push_str(...)`, compilation would instantly fail with `error[E0502]`.


## Reading the Result Carefully

A blackboard has an inspection period, an editing period, then a final inspection. Finish each access before starting the conflicting one.

| Symbol or term | Meaning |
|---|---|
| `&T` | shared access |
| `&mut T` | exclusive access |
| `last use` | helps determine borrow end |

The rule concerns overlapping active access to the same data, not one reference per whole function. Disjoint fields may be borrowed independently.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
