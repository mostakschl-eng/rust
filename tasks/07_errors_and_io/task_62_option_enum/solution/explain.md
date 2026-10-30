# Solution Explanation: Task 62 (Option Enum)

Let's dissect how returning `Option<T>` clearly communicates that an operation may yield a missing value!

---

## 🔍 The Solution Code

```rust
fn find_player_score(player_id: u32) -> Option<u32> {
    match player_id {
        101 => Some(950),
        102 => Some(820),
        _ => None,
    }
}
```

---

## 🧩 Key Takeaways

1. **Clear Function Signature:**
   * Anyone calling `find_player_score` knows from the return type `Option<u32>` that the player might not be registered.
2. **`.is_some()` vs `match`:**
   * `.is_some()` is convenient for quick boolean checks.
   * `match` unpacks the inner value cleanly.

