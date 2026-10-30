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
   * The shared borrow is no longer needed after its last use; shelf itself remains alive.
2. **Safe Mutation:**
   * Calling `shelf.push(...)` on line 10 requires a mutable borrow `&mut shelf`.
   * Because the earlier read-only borrow is dead, the mutable borrow succeeds with 0 errors!


## Reading the Result Carefully

An inspector returns a key after their final inspection; the room need not remain locked until the whole shift ends.

| Symbol or term | Meaning |
|---|---|
| `NLL` | control-flow-based borrow regions |
| `&shelf[0]` | borrow element |
| `.push()` | mutable access to vector |

Ending a borrow is not dropping the owner. Adding a later use of active_potion after push revives the conflict.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
