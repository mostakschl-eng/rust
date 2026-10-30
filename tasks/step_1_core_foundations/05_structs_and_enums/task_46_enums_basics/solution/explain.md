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

## Everyday picture and symbols

Think of an order label with one status selected at a time. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `enum OrderStatus` | Define possible variants |
| `OrderStatus::Pending` | Construct one variant |
| `PartialEq` | Enable comparisons such as == |
| `*self` | Look through a reference |
| `\|\|` | Logical OR |

## Common pitfall

The equality operator borrows its operands, so `*self == ...` does not move this enum out of the borrowed receiver. Enums restrict choices, not legal transitions.

## Expected output

```text
=== E-COMMERCE ORDER TRACKER ===
Order A status: Pending (Active: true)
Order B status: Delivered (Active: false)
Order C status: Cancelled (Active: false)
================================
```
