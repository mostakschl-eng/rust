# Task 23: Digital Security Vault PIN Validator (Comparison Operators)

## 🎯 What is Your Task?
Write a Rust program simulating a **Digital Vault Keypad Validator** using comparison operators (`==`, `!=`, `<`, `>=`).

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
3. Compare if attempts have reached warning threshold:
   - Check if `attempt_count != max_attempts`

---

## 📋 Expected Output on Your Screen
```text
=== DIGITAL SECURITY VAULT ===
Checking PIN:          Validating...
PIN Match Result:      true
Attempts Remaining:    true
Vault Access Granted! Door opening... 🔓
==============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_23_comparison_operators/src/main.rs).
2. Write comparisons using `==`, `<=`, and `!=`.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_23_comparison_operators
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Equality comparison</b></summary>

```rust
let is_match = entered_pin == correct_pin;
let has_attempts = attempt_count <= max_attempts;
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_23_comparison_operators/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_23_comparison_operators/solution/explain.md)

