# Solution Explanation: Task 33 (Arrays)

Let's review the fundamental rules of Rust arrays!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== WEATHER STATION 5-DAY LOG ===");

    let weekdays: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let temperatures: [i32; 5] = [28, 31, 30, 27, 29];

    println!("{}: {}°C", weekdays[0], temperatures[0]);
    println!("{}: {}°C", weekdays[4], temperatures[4]);
    println!("Total days logged: {}", weekdays.len());

    let sensor_status = [1; 5];
    println!("Sensor Check: {:?}", sensor_status);

    println!("=================================");
}
```

---

## 🧩 Key Takeaways

1. **Fixed Size on Stack:**
   * Both `weekdays` and `temperatures` have length 5 known at compile time.
   * `[Type; Size]` specifies the type and the element count.
2. **Zero-Based Indexing:**
   * First item: `[0]`
   * Fifth (and last) item: `[4]` (not `[5]`).
3. **Repetition Syntax `[value; count]`:**
   * `[1; 5]` creates `[1, 1, 1, 1, 1]`.
   * Extremely useful when initializing fixed buffers!

