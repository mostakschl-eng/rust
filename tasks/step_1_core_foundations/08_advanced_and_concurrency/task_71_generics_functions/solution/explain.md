# Solution Explanation: Task 71 (Generic Functions)

Let's dissect how generic type parameters enable type-safe, reusable logic with zero runtime overhead!

---

## 🔍 The Solution Code

```rust
fn inspect_cargo<T: std::fmt::Debug>(category: &str, item: T) {
    println!("[CARGO SCANNER: {}] Item Details: {:?}", category, item);
}
```

---

## 🧩 Key Takeaways

1. **`<T: std::fmt::Debug>`:**
   * Declares `T` as a generic placeholder.
   * Constrains `T` so that only types implementing `Debug` (printable via `{:?}`) can be passed in.
2. **Monomorphization in Action:**
   * At compile time, `rustc` creates four optimized machine code versions of `inspect_cargo`: one for `i32`, one for `f64`, one for `&str`, and one for `bool`.
   * Fast, direct execution with zero pointer indirection!

