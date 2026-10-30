# Task 25: E-Commerce Free Shipping Calculator (`if` Expression)

## 🎯 What is Your Task?
Write a Rust program that calculates customer shipping fees using an **`if` expression assignment**.

Rules:
1. `order_total: f64` = `65.50`
2. If `order_total >= 50.00`, shipping is **`0.00`** (Free Shipping!)
3. Otherwise, standard shipping is **`7.99`**.
4. Assign the fee directly from an `if` expression in one `let` declaration.
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

1. Open [`src/main.rs`](src/main.rs).
2. Assign `shipping_fee` using `let shipping_fee = if ... else ...;`.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/03_logic_and_loops/task_25_if_expressions
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

The decision itself should produce the fee. Both possible results must be floats.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Leave the final fee expression in each branch without a semicolon. Finish the enclosing let declaration with a semicolon and add the fee to the order total.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

