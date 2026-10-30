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

1. Open [`src/main.rs`](src/main.rs).
2. Implement methods using `&self` for reading and `&mut self` for modifications.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_44_struct_methods
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Read methods use `&self`; transaction methods need `&mut self`.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Reject non-positive/non-finite amounts before updating. On a valid withdrawal, check funds first, subtract, and return true; otherwise return false.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

## Input rules

Accept only positive finite transaction amounts. A rejected withdrawal returns `false` and changes nothing. A rejected deposit prints a rejection and returns without changing the balance. Also reject a deposit if the resulting balance is not finite. This is a learning model using `f64`; production money needs exact units such as integer cents.

## Watch out

Without amount validation, withdrawing a negative number would increase the balance. Methods do not automatically hide fields from code in the same module.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
