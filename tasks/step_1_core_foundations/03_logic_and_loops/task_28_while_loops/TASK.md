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
5. After the loop exits, print `"SECURITY ALERT: Maximum attempts exceeded! Account Locked! 🔒"`

---

## 📋 Expected Output on Your Screen
```text
SECURITY SYSTEM: Monitoring login attempts...
Attempt 1 of 3: Incorrect PIN! ❌
Attempt 2 of 3: Incorrect PIN! ❌
Attempt 3 of 3: Incorrect PIN! ❌
SECURITY ALERT: Maximum attempts exceeded! Account Locked! 🔒
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_28_while_loops/src/main.rs).
2. Write the `while attempts_used < max_allowed { ... }` loop.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_28_while_loops
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: While loop structure</b></summary>

```rust
while attempts_used < max_allowed {
    attempts_used += 1;
    println!("Attempt {} of {}: Incorrect PIN! ❌", attempts_used, max_allowed);
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_28_while_loops/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_28_while_loops/solution/explain.md)

