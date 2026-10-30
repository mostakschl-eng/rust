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
   * They are local values, often pictured in main's stack frame; optimization can store them in registers or remove storage.
2. **Heap Memory:**
   * `bio` and `skill_levels` can expand or shrink dynamically.
   * A fixed-size handle (commonly 24 bytes on 64-bit targets) is local here, while the buffer contents (`"Rust Explorer & System Architect"` and `[85, 90, 95, 99]`) reside on the heap.


## Reading the Result Carefully

The local handle is a warehouse receipt; the buffer is the stored goods. Moving the receipt need not move all the goods.

| Symbol or term | Meaning |
|---|---|
| `String` | owned UTF-8 text |
| `Vec<T>` | growable sequence |
| `.len()` | used length |
| `.capacity()` | reserved space |

A type is not inherently a stack type or heap type. Local storage may be optimized into registers, and a fixed-size array can be boxed.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
