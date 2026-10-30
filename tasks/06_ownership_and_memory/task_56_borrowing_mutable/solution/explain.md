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

