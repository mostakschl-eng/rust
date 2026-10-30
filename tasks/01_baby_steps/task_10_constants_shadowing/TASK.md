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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_10_constants_shadowing/src/main.rs).
2. Define the `const TAX_PERCENTAGE: i32 = 10;` outside of `main()`.
3. Inside `main()`, use `let price = ...` shadowing to step through the discount and tax.
4. Test with:
   ```bash
   cd tasks/01_baby_steps/task_10_constants_shadowing
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Shadowing pattern</b></summary>

```rust
const TAX_PERCENTAGE: i32 = 10;

fn main() {
    let price = 100;
    let price = price - 20;
    let price = price + (price * TAX_PERCENTAGE / 100);
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_10_constants_shadowing/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_10_constants_shadowing/solution/explain.md)
