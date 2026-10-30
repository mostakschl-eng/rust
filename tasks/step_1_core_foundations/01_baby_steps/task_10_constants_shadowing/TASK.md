# Task 10: Store Pricing & Tax Calculator (Constants & Shadowing)

## 🎯 What is Your Task?
Write a Rust program that uses both **`const`** and **variable shadowing** to calculate a customer's shopping cart total with tax and discount.

Requirements:
1. Define a global constant:
   `const TAX_PERCENTAGE: i32 = 10;` (10% sales tax)
2. In `main()`, declare:
   `let price = 100;` (Initial item price: $100)
3. Shadow `price` by applying a $20 discount coupon:
   `let price = price - 20;`
4. Shadow `price` again by calculating the final price including tax:
   `let price = price + (price * TAX_PERCENTAGE / 100);`
5. Print all stages of the price calculation!

---

## 📋 Expected Output on Your Screen
```text
=== SUPERSTORE CHECKOUT ===
Tax Rate:             10%
Original Price:       $100
Price After Coupon:   $80
Final Price With Tax: $88
===========================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Define the `const TAX_PERCENTAGE: i32 = 10;` outside of `main()`.
3. Inside `main()`, use `let price = ...` shadowing to step through the discount and tax.
4. Test with:
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_10_constants_shadowing
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

The tax percentage is a typed constant; each transformed price is a new let binding with the same name.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Print the original price before shadowing it. Discount first, then calculate the tax on the discounted price.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

