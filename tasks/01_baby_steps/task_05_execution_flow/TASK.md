# Task 05: Rocket Launch Sequence (Execution Flow)

## 🎯 What is Your Task?
Write a Rust program simulating a **Rocket Launch Sequence** using three small functions:

1. `check_fuel()`: Prints `"Fuel tanks at 100% capacity."`
2. `check_engines()`: Prints `"All engines online and ready."`
3. `launch()`:
   - Prints `"--- Pre-flight checks starting ---"`
   - Calls `check_fuel()`
   - Calls `check_engines()`
   - Prints `"--- Liftoff in 3... 2... 1... BLASTOFF! 🚀 ---"`
4. In `main()`, print `"Mission Control: Initiating Launch Protocol"` and then call `launch()`.

---

## 📋 Expected Output on Your Screen
```text
Mission Control: Initiating Launch Protocol
--- Pre-flight checks starting ---
Fuel tanks at 100% capacity.
All engines online and ready.
--- Liftoff in 3... 2... 1... BLASTOFF! 🚀 ---
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_05_execution_flow/src/main.rs).
2. Define the three helper functions.
3. Call them in the correct sequence.
4. Test with:
   ```bash
   cd tasks/01_baby_steps/task_05_execution_flow
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Function calling another function</b></summary>

Inside `fn launch()`, simply type the other function names followed by `();`:
```rust
fn launch() {
    check_fuel();
    check_engines();
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_05_execution_flow/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_05_execution_flow/solution/explain.md)
