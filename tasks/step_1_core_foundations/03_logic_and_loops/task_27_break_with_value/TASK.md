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

1. Open [`src/main.rs`](src/main.rs).
2. Assign `years_taken` directly using `let years_taken = loop { ... break years; };`.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_27_break_with_value
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Track both years and balance, increasing each once per annual deposit. The loop must produce the answer for years_taken.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

When balance reaches or exceeds the goal, exit while carrying the current years value. Finish the let declaration after the loop and print its result.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

