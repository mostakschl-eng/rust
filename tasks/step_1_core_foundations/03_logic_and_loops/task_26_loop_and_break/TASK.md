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

1. Open [`src/main.rs`](src/main.rs).
2. Write the `loop` block with `battery_level += 10;` and `break;`.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_26_loop_and_break
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

After each simulated charge, update the battery and print the new percentage. A loop with an unconditional break stops after only one charge.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Check for the full battery inside the loop and break only when that condition holds. Starting at 70 should produce three charging messages.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

