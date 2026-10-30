# Solution Explanation: Task 44 (Struct Methods)

Let's dissect how methods encapsulate state and behavior inside a Rust struct!

---

## 🔍 The Solution Code

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn display(&self) {
        println!("Account Holder: {} | Balance: ${:.2}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: f64) {
        if !amount.is_finite() || amount <= 0.0 || !(self.balance + amount).is_finite() {
            println!("Deposit rejected: amount must be positive and keep balance finite.");
            return;
        }
        self.balance += amount;
        println!("Deposited ${:.2} successfully.", amount);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if amount.is_finite() && amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}
```

---

## 🧩 Key Takeaways

1. **`&self` vs `&mut self`:**
   * `display(&self)` only reads values, so it borrows immutably with `&self`.
   * `deposit(&mut self, ...)` and `withdraw(&mut self, ...)` change `self.balance`, so they require `&mut self`.
2. **Encapsulation:**
   * The rules for withdrawal (checking if funds exist) are protected inside the method. The method rejects an overdraft and invalid amounts. Code in this same module can still directly edit the fields; methods alone do not enforce access control.

## Everyday picture and symbols

Think of an ATM with separate balance-reading and balance-changing controls. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `impl BankAccount` | Define associated behavior |
| `&self` | Borrow to inspect |
| `&mut self` | Borrow exclusively to modify |
| `-> bool` | Return success or failure |

## Common pitfall

Without amount validation, withdrawing a negative number would increase the balance. Methods do not automatically hide fields from code in the same module.

## Expected output

```text
=== SMART ATM BANKING SYSTEM ===
Account Holder: Tariq Ali | Balance: $200.00
Deposited $150.00 successfully.
Withdrew $50.00 successfully.
⚠️ Overdraft Alert: Insufficient funds for $500.00!
Account Holder: Tariq Ali | Balance: $300.00
================================
```

## Reading the amount checks

`amount.is_finite()` is true for ordinary finite floating-point numbers and false for NaN (“not a number”) and positive or negative infinity. `!` reverses a boolean, `||` means “or,” and `&&` means “and.” The deposit rejects an invalid amount or a result that would overflow to infinity, then returns before changing the account. The withdrawal accepts only a positive finite amount that the balance covers. A zero amount is deliberately rejected too. Use these methods with a finite, non-negative starting balance as in this exercise.
