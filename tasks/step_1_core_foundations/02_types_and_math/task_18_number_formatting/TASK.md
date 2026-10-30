# Task 18: Gourmet Cafe Receipt (Precision & Formatting)

## 🎯 What is Your Task?
Write a Rust program simulating a **Gourmet Coffee Shop Receipt Table** using advanced formatting.

Format requirements:
1. Column 1 (Item Name): Left-aligned in 16 character positions (`{:<16}`)
2. Column 2 (Price): Right-aligned in 7 character positions after a dollar sign, with exactly 2 decimals (`{:>7.2}`)
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

1. Open [`src/main.rs`](src/main.rs).
2. Use format specifiers like `{:<16}` and `{:>7.2}`.
3. Test with:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_18_number_formatting
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Column widths are minimum widths. Give item text a left-aligned field and amounts a right-aligned field with two decimal places.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

The solution uses 16 positions for labels, then a dollar sign and a 7-position amount field. Compute subtotal and tax from the unformatted values.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

