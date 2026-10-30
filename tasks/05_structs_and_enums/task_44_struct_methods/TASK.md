# Task 44: Smart ATM & Bank Account (Struct Methods)

## 🎯 What is Your Task?
Write a Rust program to manage a bank account using `impl` methods:
1. Define a struct `BankAccount`:
   - `owner: String`
   - `balance: f64`
2. Implement three methods inside `impl BankAccount`:
   - `display(&self)`: prints `"Account Holder: <owner> | Balance: $<balance>"`
   - `deposit(&mut self, amount: f64)`: adds amount to balance and prints confirmation.
   - `withdraw(&mut self, amount: f64) -> bool`: checks if balance has sufficient funds. If yes, subtracts and returns `true`; if insufficient, leaves balance untouched and returns `false`.
3. In `main()`:
   - Create account for `"Tariq Ali"` with `$200.00`.
   - Call `account.display()`.
   - Deposit `$150.00`.
   - Withdraw `$50.00` (succeeds).
   - Try to withdraw `$500.00` (rejected with overdraft alert).
   - Call `account.display()` for final balance.

---

## 📋 Expected Output on Your Screen
```text
=== SMART ATM BANKING SYSTEM ===
Account Holder: Tariq Ali | Balance: $200.00
Deposited $150.00 successfully.
Withdrew $50.00 successfully.
⚠️ Overdraft Alert: Insufficient funds for $500.00!
Account Holder: Tariq Ali | Balance: $300.00
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_44_struct_methods/src/main.rs).
2. Implement methods using `&self` for reading and `&mut self` for modifications.
3. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_44_struct_methods
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_44_struct_methods/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_44_struct_methods/solution/explain.md)

