# Solution Explanation: Task 46 (Enums Basics)

Let's dissect how enums prevent invalid state bugs in Rust!

---

## 🔍 The Solution Code

```rust
#[derive(Debug, PartialEq)]
enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    fn is_active(&self) -> bool {
        *self == OrderStatus::Pending || *self == OrderStatus::Shipped
    }
}
```

---

## 🧩 Key Takeaways

1. **`#[derive(PartialEq)]`:**
   * Automatically adds the ability to use the equality operator `==` between enum values (`*self == OrderStatus::Pending`).
2. **Type-Safe Domain States:**
   * An order can never be in an illegal state like `"Processingg"` or `"Error"`. Only the 4 defined variants are permitted by the compiler.
3. **Methods on Enums:**
   * Enums can have `impl` blocks just like structs!

