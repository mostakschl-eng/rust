# Solution Explanation: Task 64 (Result Enum)

Let's dissect how `Result<T, E>` replaces runtime exceptions with deterministic compile-time safety!

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
   * Instead of hiding potential division by zero bugs, the function signature `Result<f64, String>` forces the caller to write handling code for both outcomes.
2. **Graceful Fallbacks with `.unwrap_or()`:**
   * If an operation fails and a sensible default exists, `.unwrap_or(0.0)` extracts the value without crashing or requiring verbose `match` blocks.

