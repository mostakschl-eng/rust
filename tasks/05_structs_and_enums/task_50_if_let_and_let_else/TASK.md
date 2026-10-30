# Task 50: Security Terminal & Prize Scanner (if let & let else)

## 🎯 What is Your Task?
Write a Rust program demonstrating concise pattern matching with `if let` and `let else`:
1. In `main()`, inspect an optional tournament prize:
   ```rust
   let prize: Option<&str> = Some("Diamond Trophy 💎");
   ```
   - Use `if let Some(trophy) = prize` to print `"🎉 Winner Claimed: <trophy>"`.
   - Provide an `else` block printing `"No prize awarded."`.
2. Write a security verification function:
   ```rust
   fn verify_clearance(badge_id: Option<u32>)
   ```
   - Use **`let else`** to extract the badge ID into a local variable:
     ```rust
     let Some(id) = badge_id else {
         println!("⛔ Access Denied: No badge detected. Incident logged.");
         return;
     };
     ```
   - After the guard, print: `"🔑 Access Approved for Officer #<id>!"`.
3. In `main()`, call `verify_clearance(Some(9042))` and `verify_clearance(None)`.

---

## 📋 Expected Output on Your Screen
```text
=== CONCISE MATCHING TERMINAL ===
🎉 Winner Claimed: Diamond Trophy 💎

=== SECURITY CLEARANCE CHECKPOINT ===
Attempt 1 (Valid Badge):
🔑 Access Approved for Officer #9042!

Attempt 2 (Missing Badge):
⛔ Access Denied: No badge detected. Incident logged.
=====================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_50_if_let_and_let_else/src/main.rs).
2. Implement `if let` for the prize scanner.
3. Implement `let else` with early `return;` inside `verify_clearance`.
4. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_50_if_let_and_let_else
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_50_if_let_and_let_else/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_50_if_let_and_let_else/solution/explain.md)

