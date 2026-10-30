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
        self.balance += amount;
        println!("Deposited ${:.2} successfully.", amount);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
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
   * The rules for withdrawal (checking if funds exist) are protected inside the method. Outside code cannot accidentally deduct more money than exists!

