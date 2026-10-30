# Task 48: Multi-Gateway Payment Terminal (Exhaustive Match)

## 🎯 What is Your Task?
Write a Rust program to process customer checkout payments using exhaustive pattern matching:
1. Define an enum `PaymentMethod`:
   - `Cash`
   - `CreditCard(String)`: holding card number
   - `Crypto { wallet: String, coin: String }`: holding wallet address and cryptocurrency ticker
2. Write a function `process_payment(method: PaymentMethod)` that matches every variant:
   - `Cash` -> prints `"Payment via Cash received. Receipt printed. 💵"`
   - `CreditCard(num)` -> prints `"Processing Credit Card [****{last_4}]... Approved! 💳"`
   - `Crypto { wallet, coin }` -> prints `"Broadcasting {coin} transfer to wallet {wallet}... Confirmed on blockchain! ⛓️"`
3. In `main()`, call `process_payment` for all three payment methods!

---

## 📋 Expected Output on Your Screen
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

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write the exhaustive `match` covering every single variant.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_48_match_exhaustive
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Write an explicit arm for each variant so the compiler can check coverage.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Bind the card payload as `num`, validate its ASCII digits and length before taking the last four bytes, and unpack Crypto’s named fields.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

## Input rule and simulation

Before slicing a card number, reject inputs with fewer than four bytes or any non-ASCII digit. Print `Invalid demo card number: use at least four ASCII digits.` and return. All payment messages are a simulation: no payment, receipt, network request, or blockchain confirmation occurs.

## Watch out

An exhaustive match handles every shape, but does not prove business logic or real payment success. The messages only simulate payment handling.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
