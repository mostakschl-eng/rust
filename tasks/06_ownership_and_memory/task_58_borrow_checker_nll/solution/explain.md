# Solution Explanation: Task 58 (Non-Lexical Lifetimes NLL)

Let's dissect how the modern Borrow Checker calculates reference lifetimes based on usage rather than scopes!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== ALCHEMIST POTION SHELF ===");

    let mut shelf = vec!["Health Potion", "Mana Potion"];

    let active_potion = &shelf[0];
    println!("Currently inspecting: {}", active_potion);

    println!("Adding new rare potion to shelf...");
    shelf.push("Elixir of Life");

    println!("Shelf Inventory: {:?}", shelf);

    println!("==============================");
}
```

---

## 🧩 Key Takeaways

1. **How NLL Works:**
   * `active_potion` is borrowed from `shelf`.
   * The compiler traces its usage: it is used on line 7 in `println!`.
   * After line 7, `active_potion` is never referenced again.
   * Modern Rust drops the borrow immediately!
2. **Safe Mutation:**
   * Calling `shelf.push(...)` on line 10 requires a mutable borrow `&mut shelf`.
   * Because the earlier read-only borrow is dead, the mutable borrow succeeds with 0 errors!

