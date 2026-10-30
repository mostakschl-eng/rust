# Task 23: Digital Security Vault PIN Validator (Comparison Operators)

## 🎯 What is Your Task?
Write a Rust program simulating a **Digital Vault Keypad Validator** using comparison operators (`==`, `!=`, `<=`).

Setup:
1. `correct_pin: &str` = `"9876"`
2. `entered_pin: &str` = `"9876"`
3. `attempt_count: i32` = `1`
4. `max_attempts: i32` = `3`

Conditions to test:
1. Compare `entered_pin == correct_pin`:
   - If true: print `"Vault Access Granted! Door opening... 🔓"`
2. Check if attempts are within limits:
   - Compare `attempt_count <= max_attempts`
3. Check whether the current attempt is not exactly the limit:
   - Compare `attempt_count != max_attempts`. This is not the same question as being within the limit.

---

## 📋 Expected Output on Your Screen
```text
=== DIGITAL SECURITY VAULT ===
Checking PIN:          Validating...
PIN Match Result:      true
Within Attempt Limit:  true
Not At Attempt Limit:  true
Vault Access Granted! Door opening... 🔓
==============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write comparisons using `==`, `<=`, and `!=`.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_23_comparison_operators
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Equality uses two equals signs. Each comparison produces a bool that can be printed.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Compare the current attempt with the maximum using <= for within the limit and != for not exactly at the limit; print both checks.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

