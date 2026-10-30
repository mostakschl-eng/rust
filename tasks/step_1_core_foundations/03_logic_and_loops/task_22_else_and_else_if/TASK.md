# Task 22: Cinema Ticket Booth (The `else if` Ladder)

## 🎯 What is Your Task?
Write a Rust program simulating a **Cinema Ticket Pricing Kiosk** using an `if / else if / else` ladder.

Pricing rules based on customer `age`:
1. If `age < 5`: Ticket is **FREE ($0)** ("Toddler: Free Admission")
2. Else if `age <= 12`: Child Ticket is **$8**
3. Else if `age <= 64`: Adult Standard Ticket is **$15**
4. Else: Senior Citizen Discount Ticket is **$10**

Test with `let age = 68;` (Senior citizen) and print the ticket category and price.

---

## 📋 Expected Output on Your Screen
```text
=== CINEMA TICKET KIOSK ===
Customer Age:       68 years
Ticket Category:    Senior Citizen
Ticket Price:       $10
Enjoy the movie! 🍿
===========================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_22_else_and_else_if/src/main.rs).
2. Write the `if / else if / else` decision ladder.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_22_else_and_else_if
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Ladder structure</b></summary>

```rust
if age < 5 {
    // Free
} else if age <= 12 {
    // Child $8
} else if age <= 64 {
    // Adult $15
} else {
    // Senior $10
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_22_else_and_else_if/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_22_else_and_else_if/solution/explain.md)

