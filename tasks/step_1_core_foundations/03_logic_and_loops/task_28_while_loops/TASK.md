# Task 28: Security PIN Brute-Force Lockout (`while` Loop)

## 🎯 What is Your Task?
Write a Rust program simulating an **Automated Security Lockout System** using a `while` loop.

Requirements:
1. `let mut attempts_used = 0;`
2. `let max_allowed = 3;`
3. Print `"SECURITY SYSTEM: Monitoring login attempts..."`
4. Use a `while attempts_used < max_allowed` loop:
   - Increment `attempts_used += 1;`
   - Print `"Attempt {} of {}: Incorrect PIN! ❌"`
5. After the loop exits, print `"SECURITY ALERT: Maximum attempts reached! Account Locked! 🔒"`

---

## 📋 Expected Output on Your Screen
```text
SECURITY SYSTEM: Monitoring login attempts...
Attempt 1 of 3: Incorrect PIN! ❌
Attempt 2 of 3: Incorrect PIN! ❌
Attempt 3 of 3: Incorrect PIN! ❌
SECURITY ALERT: Maximum attempts reached! Account Locked! 🔒
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write the `while attempts_used < max_allowed { ... }` loop.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_28_while_loops
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

The condition is checked before each attempt. Increase the used count inside the body so it eventually reaches the limit.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

An unconditional break ends the exercise after one attempt. Let the while condition stop it at three and put the lockout message after the loop.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

