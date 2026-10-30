# Task 18: Gourmet Cafe Receipt (Precision & Formatting)

## 🎯 What is Your Task?
Write a Rust program simulating a **Gourmet Coffee Shop Receipt Table** using advanced formatting.

Format requirements:
1. Column 1 (Item Name): Left-aligned in 15 spaces (`{:<15}`)
2. Column 2 (Price): Right-aligned in 8 spaces, rounded to exactly 2 decimals (`{:>8.2}`)
3. Print these 3 menu items:
   - `"Espresso"`, `2.75`
   - `"Caramel Latte"`, `4.899` (Rounds to `4.90`)
   - `"Croissant"`, `3.2` (Shows `3.20`)
4. Calculate subtotal, 8% tax (`subtotal * 0.08`), and final total rounded to 2 decimals.

---

## 📋 Expected Output on Your Screen
```text
========================
ITEM               PRICE
========================
Espresso        $   2.75
Caramel Latte   $   4.90
Croissant       $   3.20
------------------------
Subtotal:       $  10.85
Tax (8%):       $   0.87
TOTAL DUE:      $  11.72
========================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_18_number_formatting/src/main.rs).
2. Use format specifiers like `{:<15}` and `{:>7.2}`.
3. Test with:
   ```bash
   cd tasks/02_types_and_math/task_18_number_formatting
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Aligning currency and decimals</b></summary>

```rust
println!("{:<15} $ {:>6.2}", "Espresso", 2.75);
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_18_number_formatting/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_18_number_formatting/solution/explain.md)

