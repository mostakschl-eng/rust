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

1. Open [`src/main.rs`](src/main.rs).
2. Write the `if / else if / else` decision ladder.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_22_else_and_else_if
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Check the narrowest low-age groups first so they do not get captured by an adult rule.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Test ages 4, 5, 12, 13, 64, and 65 mentally against the branch order. With 68 only the senior branch should run.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

