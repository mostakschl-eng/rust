# Task 15: Egg Farm Packaging Calculator (Arithmetic & `%`)

## 🎯 What is Your Task?
Write a Rust program simulating a **Farm Egg Packaging Calculator**.

A farmer collects **`58` fresh eggs** from the hen house.
Eggs are packed into cartons of **`12` (one dozen)**.

Calculate:
1. `total_eggs: i32` = `58`
2. `carton_size: i32` = `12`
3. `full_cartons: i32` = `total_eggs / carton_size` (Integer division)
4. `leftover_eggs: i32` = `total_eggs % carton_size` (Modulus remainder)

Print a neat packaging report!

---

## 📋 Expected Output on Your Screen
```text
=== SUNNYVALE EGG FARM PACKAGING ===
Total Eggs Collected: 58
Carton Size:          12 eggs per carton
Full Cartons Packed:  4 cartons (48 eggs total)
Leftover Eggs:        10 loose eggs
====================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_15_math_operations/src/main.rs).
2. Declare `total_eggs = 58` and `carton_size = 12`.
3. Use `/` for `full_cartons` and `%` for `leftover_eggs`.
4. Print the report.
5. Test with:
   ```bash
   cd tasks/02_types_and_math/task_15_math_operations
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Math operators</b></summary>

```rust
let full_cartons = total_eggs / carton_size;
let leftover_eggs = total_eggs % carton_size;
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_15_math_operations/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_15_math_operations/solution/explain.md)

