# Task 37: Invoice Calculator & Discount Engine (Vector Iteration)

## 🎯 What is Your Task?
Write a Rust program to calculate an invoice and apply discounts:
1. Declare a mutable vector of item prices:
   ```rust
   let mut prices = vec![12.0, 45.5, 9.99, 120.0];
   ```
2. Loop over `&prices` to calculate the total sum of all items before discount, and print:
   `"Original Total: $187.49"` (formatted to 2 decimal places).
3. Loop over `&mut prices` to apply a 10% discount on every item (`*price *= 0.90;`).
4. Loop over `prices.iter().enumerate()` to print a numbered receipt with the new prices:
   `"Item #1: $10.80"`, etc.

---

## 📋 Expected Output on Your Screen
```text
=== INVOICE & DISCOUNT ENGINE ===
Original Total: $187.49

Applying 10% holiday discount to all items...
=== DISCOUNTED RECEIPT ===
Item #1: $10.80
Item #2: $40.95
Item #3: $8.99
Item #4: $108.00
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Calculate sum with `for price in &prices { sum += price; }`.
3. Apply discount with `for price in &mut prices { *price *= 0.90; }`.
4. Enumerate with `for (idx, price) in prices.iter().enumerate() { ... }`.
5. Test:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_37_vector_iteration
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Use a shared borrow to total the prices so the vector stays available.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

For the discount, loop over `&mut prices` and write `*price *= 0.90`; enumerate the receipt and display `index + 1`.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

Multiplying by 0.90 keeps 90% of the price, a 10% discount. f64 models prices for practice; real accounting needs exact units and an explicit rounding policy.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
