# Task 76: Price & Tax Calculator using Closures

## 🎯 What is Your Task?
Write a program that uses inline anonymous functions (closures) to calculate item pricing while capturing rates from the surrounding environment:

1. In `main()`, define two outer variables:
   ```rust
   let discount_rate = 0.10; // 10% off
   let tax_rate = 0.08;      // 8% tax
   ```
2. Create single-line closure `apply_discount`:
   - Takes `price: f64` as input.
   - Captures `discount_rate` from the outer scope.
   - Computes `price * (1.0 - discount_rate)`.
3. Create single-line closure `apply_tax`:
   - Takes `price: f64` as input.
   - Captures `tax_rate` from the outer scope.
   - Computes `price * (1.0 + tax_rate)`.
4. Create a multi-line closure `format_bill`:
   - Takes `price: f64`.
   - Uses `apply_discount` and `apply_tax` in sequence to compute the final bill.
   - Returns the final amount.
5. Calculate and display the original price (`$150.00`), discounted price, and final bill!

---

## 📋 Expected Output on Your Screen
```text
=== BILL CALCULATOR WITH CLOSURES ===
Original Price:  $150.00
Discounted:      $135.00
Final Bill:      $145.80
=====================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write the closure signatures using vertical pipes `|price: f64|`.
3. Capture outer variables directly inside the closure body.
4. Run:
   ```bash
   cd tasks/step_1_core_foundations/08_advanced_and_concurrency/task_76_closures
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Capture each rate in its corresponding single-line closure.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Inside format_bill, apply the discount first and tax second; return the final expression without a semicolon.

</details>

**Before running:** Fn, FnMut, and FnOnce overlap. move chooses capture mode, not a one-call limit. Floating-point prices here teach closures, not exact accounting.
