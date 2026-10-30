# Solution Explanation: Task 52 (Three Ownership Rules)

Let's dissect how Rust enforces scope-based deallocation and exclusive single ownership!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== SACRED OWNERSHIP DEMO ===");

    println!("Entering temporary room...");
    {
        let temporary_pass = String::from("VIP-ACCESS-PASS-2024");
        println!("Inside inner scope: {}", temporary_pass);
    }
    println!("Left temporary room! (VIP pass automatically dropped by Rust)");

    println!("\nOwnership Transfer Relay:");
    let original_owner = String::from("Championship Cup 🏆");
    let new_owner = original_owner;

    println!("Trophy successfully transferred to new owner: {}", new_owner);

    println!("=============================");
}
```

---

## 🧩 Key Takeaways

1. **Automatic Dropping (`{ ... }`):**
   * The moment the closing brace `}` was reached, Rust called `drop` on `temporary_pass`. The heap memory was immediately reclaimed.
2. **Move Semantics:**
   * When `let new_owner = original_owner;` ran, ownership moved.
   * `original_owner` became invalid.
   * Only `new_owner` is responsible for dropping the memory at the end of `main`.

