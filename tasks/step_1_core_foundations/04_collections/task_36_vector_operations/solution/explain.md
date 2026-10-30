# Solution Explanation: Task 36 (Vector Operations)

Let's break down how to manage mutable vectors like a stack or shopping cart!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== GROCERY CART MANAGER ===");

    let mut cart = Vec::new();

    cart.push("Apples");
    cart.push("Bread");
    cart.push("Milk");

    println!("Added items! Cart count: {}", cart.len());

    match cart.pop() {
        Some(item) => println!("Popped item from cart: {}", item),
        None => println!("Cart is empty!"),
    }

    println!("Remaining items: {:?}", cart);

    println!("Clearing all items...");
    cart.clear();

    println!("Cart is empty? {}", cart.is_empty());
    if cart.is_empty() {
        println!("Cart successfully cleared! 🛒");
    }

    println!("============================");
}
```

---

## 🧩 Key Takeaways

1. **`push()` appends to the end:**
   * Adding items is $O(1)$ amortized time.
2. **`pop()` returns `Option<T>`:**
   * Since popping an empty vector cannot return a valid item, Rust returns `Option<T>` instead of throwing an exception or returning `null`.
3. **`clear()` vs reallocating:**
   * `clear()` drops all elements, setting `len` to 0, but retains the allocated memory capacity for future pushes without reallocating!

## Everyday picture and symbols

Think of a cafeteria dispenser where the last tray added is removed first. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `.push(x)` | Append at the end |
| `.pop()` | Remove last element as Option<T> |
| `.clear()` | Drop all elements but retain capacity |
| `.is_empty()` | Check whether length is zero |

## Common pitfall

`pop()` removes an owned value; `get()` only borrows. Clearing does not necessarily release the allocation.

## Expected output

```text
=== GROCERY CART MANAGER ===
Added items! Cart count: 3
Popped item from cart: Milk
Remaining items: ["Apples", "Bread"]
Clearing all items...
Cart is empty? true
Cart successfully cleared! 🛒
============================
```
