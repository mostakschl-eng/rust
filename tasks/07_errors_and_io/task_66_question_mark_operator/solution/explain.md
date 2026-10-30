# Solution Explanation: Task 66 (? Operator)

Let's dissect how the `?` operator collapses multi-step error handling into clean, readable code!

---

## 🔍 The Solution Code

```rust
use std::num::ParseIntError;

fn calculate_volume(w_str: &str, h_str: &str, d_str: &str) -> Result<i32, ParseIntError> {
    let w: i32 = w_str.parse()?;
    let h: i32 = h_str.parse()?;
    let d: i32 = d_str.parse()?;

    Ok(w * h * d)
}
```

---

## 🧩 Key Takeaways

1. **How `?` works:**
   * `w_str.parse()?`: If `parse()` returns `Ok(val)`, `val` is extracted into `w`.
   * If `h_str.parse()?` fails on `"invalid_height"`, it immediately returns `Err(ParseIntError)` out of the entire `calculate_volume` function!
   * The third line `d_str.parse()?` is never even reached.
2. **Readability:**
   * Code reads like standard linear execution, with error propagation handled entirely behind the scenes!

