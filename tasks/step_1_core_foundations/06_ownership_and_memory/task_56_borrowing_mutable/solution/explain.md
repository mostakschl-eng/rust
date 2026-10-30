# Solution Explanation: Task 56 (Mutable Borrowing)

Let's dissect how exclusive mutable borrowing modifies memory in place without moving ownership!

---

## 🔍 The Solution Code

```rust
fn add_alert_badge(msg: &mut String) {
    msg.push_str(" [CRITICAL LEVEL 5 ALERT 🚨]");
}

fn boost_voltage(voltage: &mut f64, multiplier: f64) {
    *voltage *= multiplier;
}
```

---

## 🧩 Key Takeaways

1. **`&mut String`:**
   * Grants the function permission to grow or alter the heap buffer of `system_status`.
   * Ownership never leaves `main`.
2. **The Dereference Operator `*`:**
   * `voltage` is a reference `&mut f64` (a pointer).
   * To multiply the actual number residing at that memory address, we write `*voltage *= multiplier;`.


## Reading the Result Carefully

One editing pen grants exclusive temporary access to a document; returning the pen leaves the owner with the edited document.

| Symbol or term | Meaning |
|---|---|
| `&mut T` | exclusive borrowed access |
| `*reference` | access referent |
| `*=` | multiply and assign |

A mutable reference can read as well as write. A shared reborrow or exclusive reborrow may temporarily restrict access through the original reference.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
