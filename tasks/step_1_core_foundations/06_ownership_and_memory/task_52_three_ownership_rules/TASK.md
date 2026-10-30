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

1. Open [`src/main.rs`](src/main.rs).
2. Create inner block `{ ... }`.
3. Transfer ownership with `let new_owner = original_owner;`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/06_ownership_and_memory/task_52_three_ownership_rules
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Use an inner block for the temporary pass so its owner ends before the next message.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Create the trophy outside that block, transfer it once, and print only the new owner.

</details>

**Before running:** A moved value is not an empty value. Drop releases a String buffer to its allocator, not necessarily to the OS, and does not erase secrets.
