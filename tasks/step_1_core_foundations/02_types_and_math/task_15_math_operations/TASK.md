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

1. Open [`src/main.rs`](src/main.rs).
2. Declare `total_eggs = 58` and `carton_size = 12`.
3. Use `/` for `full_cartons` and `%` for `leftover_eggs`.
4. Print the report.
5. Test with:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_15_math_operations
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Integer division answers how many full cartons fit; remainder answers how many eggs remain.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Check your result by multiplying cartons by carton size and adding the leftovers; the total should be 58.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

