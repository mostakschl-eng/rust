# Task 13: App Security Dashboard (Boolean Flags)

## 🎯 What is Your Task?
Write a Rust program simulating a **User Account Security Dashboard** using `bool` variables.

Declare these 4 security flags:
1. `is_email_verified: bool` = `true`
2. `has_two_factor_auth: bool` = `true`
3. `is_account_suspended: bool` = `false`
4. `can_make_purchases: bool` = `!is_account_suspended` (Flipped using `!`)

Print all 4 status flags!

---

## 📋 Expected Output on Your Screen
```text
=== SECURITY AUDIT DASHBOARD ===
Email Verified:        true
Two-Factor Auth:       true
Account Suspended:     false
Can Make Purchases:    true
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_13_boolean_flags/src/main.rs).
2. Declare each boolean variable.
3. Use `!is_account_suspended` to set `can_make_purchases`.
4. Print each flag.
5. Test with:
   ```bash
   cd tasks/02_types_and_math/task_13_boolean_flags
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Boolean flip</b></summary>

```rust
let is_account_suspended: bool = false;
let can_make_purchases: bool = !is_account_suspended;
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_13_boolean_flags/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_13_boolean_flags/solution/explain.md)
