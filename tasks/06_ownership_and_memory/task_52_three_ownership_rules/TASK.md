# Task 52: The Sacred Ownership Relay (Three Rules)

## 🎯 What is Your Task?
Write a Rust program to demonstrate the Three Sacred Rules of Ownership:
1. In `main()`, create an inner scope `{ ... }`:
   - Declare `let temporary_pass = String::from("VIP-ACCESS-PASS-2024");`
   - Print `"Inside inner scope: <pass>"`
   - Close the scope and print a comment indicating `temporary_pass` has been automatically dropped!
2. Declare `let original_owner = String::from("Championship Cup 🏆");`
3. Transfer ownership to `let new_owner = original_owner;`
4. Print `"Trophy successfully transferred to new owner: <new_owner>"`

---

## 📋 Expected Output on Your Screen
```text
=== SACRED OWNERSHIP DEMO ===
Entering temporary room...
Inside inner scope: VIP-ACCESS-PASS-2024
Left temporary room! (VIP pass automatically dropped by Rust)

Ownership Transfer Relay:
Trophy successfully transferred to new owner: Championship Cup 🏆
=============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_52_three_ownership_rules/src/main.rs).
2. Create inner block `{ ... }`.
3. Transfer ownership with `let new_owner = original_owner;`.
4. Test:
   ```bash
   cd tasks/06_ownership_and_memory/task_52_three_ownership_rules
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_52_three_ownership_rules/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_52_three_ownership_rules/solution/explain.md)

