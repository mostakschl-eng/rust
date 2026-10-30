# Solution Explanation: Task 48 (Exhaustive Match)

Let's dissect how exhaustive pattern matching ensures zero unhandled cases in transaction systems!

---

## 🔍 The Solution Code

```rust
#[allow(dead_code)]
enum PaymentMethod {
    Cash,
    CreditCard(String),
    Crypto { wallet: String, coin: String },
}

fn process_payment(method: PaymentMethod) {
    match method {
        PaymentMethod::Cash => {
            println!("Payment via Cash received. Receipt printed. 💵");
        }
        PaymentMethod::CreditCard(num) => {
            let last_4 = &num[num.len() - 4..];
            println!("Processing Credit Card [****{}]... Approved! 💳", last_4);
        }
        PaymentMethod::Crypto { wallet, coin } => {
            println!("Broadcasting {} transfer to wallet {}... Confirmed on blockchain! ⛓️", coin, wallet);
        }
    }
}
```

---

## 🧩 Key Takeaways

1. **Exhaustiveness:**
   * Every arm handles a specific variant. If you add `PaymentMethod::ApplePay` in the future, the compiler will refuse to compile until you explicitly add an arm here!
2. **Unpacking In-Place:**
   * `CreditCard(num)` extracts the card number string into `num`.
   * `Crypto { wallet, coin }` extracts both fields into local variables automatically.

