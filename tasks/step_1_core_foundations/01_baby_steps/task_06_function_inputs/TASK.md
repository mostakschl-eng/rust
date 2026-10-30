# Task 06: Product Receipt Line Printer (Function Inputs)

## 🎯 What is Your Task?
Write a Rust program with a reusable function named **`print_item()`** that accepts **three parameters**:
1. `item_name: &str` (Name of product, e.g. `"Apple"`)
2. `quantity: i32` (Number of items bought, e.g. `4`)
3. `price: i32` (Price per item in dollars, e.g. `2`)

Inside `print_item()`, print a formatted line showing all three pieces of data using placeholders. To match the aligned sample, `{:<10}` is a preview of Lesson 18: left-align text in a field at least 10 characters wide.
In `main()`, call `print_item()` for at least **three different items**!

---

## 📋 Expected Output on Your Screen
```text
=== GROCERY RECEIPT ===
Item: Laptop     | Qty: 1 | Price: $900
Item: Headphones | Qty: 2 | Price: $50
Item: Coffee Mug | Qty: 3 | Price: $12
=======================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Define `fn print_item(item_name: &str, quantity: i32, price: i32) { ... }`.
3. In `main()`, call `print_item("Laptop", 1, 900);` and other items.
4. Test with:
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_06_function_inputs
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Give each parameter a name and type; text and whole numbers need different types.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Inside the helper, match each placeholder to its argument. Call it three times with the receipt values; left alignment can pad the names.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

