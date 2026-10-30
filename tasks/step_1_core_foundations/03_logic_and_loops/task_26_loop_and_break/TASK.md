# Task 26: EV Battery Charger Station (`loop` and `break`)

## 🎯 What is Your Task?
Write a Rust program simulating an **Electric Vehicle (EV) Rapid Charging Station** using a `loop` and `break`.

Simulation steps:
1. `let mut battery_level = 70;` (Car arrives at 70% charge)
2. Print `"EV Station: Plugged in. Fast charging started..."`
3. Enter a `loop`:
   - Increase `battery_level += 10;`
   - Print `"Charging: {}%..."`
   - When `battery_level >= 100`, print `"Battery 100% Full! Unplugging cable. ⚡"`, and call **`break;`** to exit the loop!
4. Print `"EV Station: Ready to drive!"`

---

## 📋 Expected Output on Your Screen
```text
EV Station: Plugged in. Fast charging started...
Charging: 80%...
Charging: 90%...
Charging: 100%...
Battery 100% Full! Unplugging cable. ⚡
EV Station: Ready to drive!
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_26_loop_and_break/src/main.rs).
2. Write the `loop` block with `battery_level += 10;` and `break;`.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_26_loop_and_break
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Loop with break</b></summary>

```rust
loop {
    battery_level += 10;
    println!("Charging: {}%...", battery_level);
    if battery_level >= 100 {
        println!("Battery 100% Full! Unplugging cable. ⚡");
        break;
    }
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_26_loop_and_break/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_26_loop_and_break/solution/explain.md)

