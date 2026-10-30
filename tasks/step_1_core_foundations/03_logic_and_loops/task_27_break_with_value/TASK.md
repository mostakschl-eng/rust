# Task 27: Savings Goal Investment Tracker (`break value;`)

## 🎯 What is Your Task?
Write a Rust program simulating a **Bank Savings Investment Goal** where a `loop` computes how many years it takes to reach a $1,000 target and returns the years taken using `break years;`.

Simulation rules:
1. `let mut balance = 200;` (Starting deposit: $200)
2. `let mut years = 0;`
3. Enter a loop that adds **$150** every year:
   - `years += 1;`
   - `balance += 150;`
   - When `balance >= 1000`, exit with:
     ```rust
     break years;
     ```
4. Assign the returned value: `let years_taken = loop { ... };`
5. Print the investment milestone report!

---

## 📋 Expected Output on Your Screen
```text
=== SAVINGS GOAL CALCULATOR ===
Initial Deposit: $200
Target Goal:     $1000
Calculating annual deposits of $150...
Goal Reached! Target surpassed in 6 years!
Final Balance:   $1100
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_27_break_with_value/src/main.rs).
2. Assign `years_taken` directly using `let years_taken = loop { ... break years; };`.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_27_break_with_value
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Loop return syntax</b></summary>

```rust
let years_taken = loop {
    years += 1;
    balance += 150;
    if balance >= 1000 {
        break years;
    }
};
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_27_break_with_value/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_27_break_with_value/solution/explain.md)

