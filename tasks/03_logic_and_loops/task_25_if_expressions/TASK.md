# Task 25: E-Commerce Free Shipping Calculator (`if` Expression)

## 🎯 What is Your Task?
Write a Rust program that calculates customer shipping fees using an **`if` expression assignment**.

Rules:
1. `order_total: f64` = `65.50`
2. If `order_total >= 50.00`, shipping is **`0.00`** (Free Shipping!)
3. Otherwise, standard shipping is **`7.99`**.
4. Assign the fee directly in one statement:
   ```rust
   let shipping_fee: f64 = if order_total >= 50.0 { 0.0 } else { 7.99 };
   ```
5. Calculate `final_bill = order_total + shipping_fee` and print the checkout breakdown!

---

## 📋 Expected Output on Your Screen
```text
=== ONLINE STORE CHECKOUT ===
Order Subtotal:   $65.50
Free Shipping Qualified!
Shipping Fee:     $0.00
Final Total:      $65.50
=============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_25_if_expressions/src/main.rs).
2. Assign `shipping_fee` using `let shipping_fee = if ... else ...;`.
3. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_25_if_expressions
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Both branches must return f64</b></summary>

```rust
let shipping_fee = if order_total >= 50.0 {
    0.0
} else {
    7.99
};
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_25_if_expressions/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_25_if_expressions/solution/explain.md)

