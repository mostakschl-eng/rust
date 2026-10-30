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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_48_match_exhaustive/src/main.rs).
2. Write the exhaustive `match` covering every single variant.
3. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_48_match_exhaustive
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_48_match_exhaustive/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_48_match_exhaustive/solution/explain.md)

