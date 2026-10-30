# Task 21: ATM Cash Dispenser (The `if` Statement)

## 🎯 What is Your Task?
Write a Rust program simulating an **ATM Cash Withdrawal Check** using an `if` statement.

Requirements:
1. Declare `account_balance: i32` = `500` (User has $500 in bank)
2. Declare `withdraw_amount: i32` = `200` (User requests to withdraw $200)
3. Print `"ATM: Processing withdrawal request of $200..."`
4. Use an `if` statement:
   - Check if `account_balance >= withdraw_amount`.
   - If true: calculate `remaining = account_balance - withdraw_amount`, print `"Success! Dispensing $200 cash..."`, and print `"Remaining balance: $300"`.
5. Print `"ATM: Thank you for banking with us."`

---

## 📋 Expected Output on Your Screen
```text
ATM: Processing withdrawal request of $200...
Success! Dispensing $200 cash...
Remaining balance: $300
ATM: Thank you for banking with us.
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write the `if account_balance >= withdraw_amount { ... }` block.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_21_if_statement
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Compare the requested amount to the balance. The success messages belong inside the conditional block.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Calculate remaining balance only on the successful path. The final thank-you message belongs after the block so it prints either way.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

