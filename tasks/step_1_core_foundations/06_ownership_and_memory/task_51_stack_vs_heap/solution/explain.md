# Solution Explanation: Task 51 (Stack vs Heap)

Let's dissect how memory is distributed between the Stack and the Heap!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== MEMORY ARCHITECTURE INSPECTOR ===");

    let age: i32 = 25;
    let rating: f64 = 4.9;
    let is_pro: bool = true;

    let mut bio = String::from("Rust Explorer");
    bio.push_str(" & System Architect");

    let mut skill_levels = vec![85, 90, 95];
    skill_levels.push(99);

    println!("[STACK MEMORY]");
    println!("Age:     {} years", age);
    println!("Rating:  {:.2} / 5.0", rating);
    println!("Pro Dev: {}", is_pro);

    println!("\n[HEAP MEMORY]");
    println!("Bio:     {}", bio);
    println!("Skills:  {:?} (Count: {})", skill_levels, skill_levels.len());

    println!("=====================================");
}
```

---

## 🧩 Key Takeaways

1. **Stack Memory:**
   * `age`, `rating`, and `is_pro` have fixed byte sizes (`4`, `8`, and `1` bytes).
   * They sit directly inside `main`'s stack frame.
2. **Heap Memory:**
   * `bio` and `skill_levels` can expand or shrink dynamically.
   * A 24-byte controller sits on the stack, while their contents (`"Rust Explorer & System Architect"` and `[85, 90, 95, 99]`) reside on the heap.

