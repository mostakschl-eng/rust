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
   * At compile time, Rust specializes the generic function for the concrete types used: i32, f64, &str, and bool. Optimization may inline, merge, or remove generated code.
   * This uses static dispatch; Debug formatting and printing still perform runtime work.


## Reading the Result Carefully

One scanner procedure can inspect different parcel types, provided each type supplies the required readable label.

| Symbol or term | Meaning |
|---|---|
| `<T>` | generic type parameter |
| `T: Debug` | required formatting capability |
| `{:?}` | Debug output |

A generic body may use only capabilities established by its bounds. Any T does not automatically support printing or arithmetic. Passing an owned non-Copy item by value still moves it.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
