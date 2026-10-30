# Task 06: Product Receipt Line Printer (Function Inputs)

## 🎯 What is Your Task?
Write a Rust program with a reusable function named **`print_item()`** that accepts **three parameters**:
1. `item_name: &str` (Name of product, e.g. `"Apple"`)
2. `quantity: i32` (Number of items bought, e.g. `4`)
3. `price: i32` (Price per item in dollars, e.g. `2`)

Inside `print_item()`, print a formatted line showing all three pieces of data using `{}` placeholders.
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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_06_function_inputs/src/main.rs).
2. Define `fn print_item(item_name: &str, quantity: i32, price: i32) { ... }`.
3. In `main()`, call `print_item("Laptop", 1, 900);` and other items.
4. Test with:
   ```bash
   cd tasks/01_baby_steps/task_06_function_inputs
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Function signature</b></summary>

```rust
fn print_item(item_name: &str, quantity: i32, price: i32) {
    println!("Item: {} | Qty: {} | Price: ${}", item_name, quantity, price);
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_06_function_inputs/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_06_function_inputs/solution/explain.md)

