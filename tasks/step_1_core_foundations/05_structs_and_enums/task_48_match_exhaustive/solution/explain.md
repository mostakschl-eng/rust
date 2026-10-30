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
            if num.len() < 4 || !num.bytes().all(|b| b.is_ascii_digit()) {
                println!("Invalid demo card number: use at least four ASCII digits.");
                return;
            }
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

## Why the card check matters

String ranges use byte positions. Subtracting four from a shorter length can underflow, and slicing inside a multibyte character panics. We first require at least four ASCII digits: each is exactly one byte, so the final four-byte range is valid. `bytes()` walks bytes; `all(|b| b.is_ascii_digit())` is an iterator helper meaning “every byte passes this check.” The small function `|b| ...` is called a closure; this is a preview of a later topic. This check demonstrates safe slicing, not real card verification. Approval and transfer messages only print simulated events.

## Everyday picture and symbols

Think of a coin sorter with one route for every coin kind. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `match method` | Inspect a value |
| `Pattern => action` | One match arm |
| `CreditCard(num)` | Extract a carried value |
| `{ wallet, coin }` | Destructure named fields |

## Common pitfall

An exhaustive match handles every shape, but does not prove business logic or real payment success. The messages only simulate payment handling.

## Expected output

```text
=== CHECKOUT PAYMENT TERMINAL ===
Payment 1:
Payment via Cash received. Receipt printed. 💵
Payment 2:
Processing Credit Card [****4242]... Approved! 💳
Payment 3:
Broadcasting BTC transfer to wallet 0x7a89b... Confirmed on blockchain! ⛓️
=================================
```
