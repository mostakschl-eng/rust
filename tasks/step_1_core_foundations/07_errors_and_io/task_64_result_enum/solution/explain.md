# Solution Explanation: Task 64 (Result Enum)

Let's dissect how `Result<T, E>` makes possible runtime failures explicit in the return type!

---

## 🔍 The Solution Code

```rust
fn split_bill(total_amount: f64, people_count: u32) -> Result<f64, String> {
    if people_count == 0 {
        Err(String::from("Cannot divide bill among 0 diners!"))
    } else {
        Ok(total_amount / people_count as f64)
    }
}
```

---

## 🧩 Key Takeaways

1. **Explicit Error Types:**
   * Instead of hiding potential division by zero bugs, the function signature `Result<f64, String>` tells the caller that the operation may fail; ignoring Result normally produces a warning.
2. **Graceful Fallbacks with `.unwrap_or()`:**
   * If an operation fails and a sensible default exists, `.unwrap_or(0.0)` extracts the value without crashing or requiring verbose `match` blocks.


## Reading the Result Carefully

A delivery receipt says either delivered with a parcel or failed with a reason. An error is useful information, not a free successful delivery.

| Symbol or term | Meaning |
|---|---|
| `Result<T, E>` | success or failure |
| `Ok(value)` | success |
| `Err(error)` | failure |

A fallback of zero demonstrates a method; it is not a correct payable share for a failed bill calculation. Real billing should report the error. The exercise assumes finite nonnegative amounts; f64 also has rounding limits.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
