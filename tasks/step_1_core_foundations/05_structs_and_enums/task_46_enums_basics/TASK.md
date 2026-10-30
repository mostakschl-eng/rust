# Task 46: E-Commerce Order Lifecycle (Enums)

## 🎯 What is Your Task?
Write a Rust program to model an order lifecycle using an enum:
1. Define an enum `OrderStatus` with `#[derive(Debug, PartialEq)]`:
   - `Pending`
   - `Shipped`
   - `Delivered`
   - `Cancelled`
2. Implement a method `is_active(&self) -> bool` inside `impl OrderStatus`:
   - Returns `true` if `*self == OrderStatus::Pending || *self == OrderStatus::Shipped`.
   - Returns `false` otherwise.
3. In `main()`:
   - Create `order_a = OrderStatus::Pending`.
   - Create `order_b = OrderStatus::Delivered` and `order_c = OrderStatus::Cancelled`.
   - Print their statuses and whether each order is currently active.

---

## 📋 Expected Output on Your Screen
```text
=== E-COMMERCE ORDER TRACKER ===
Order A status: Pending (Active: true)
Order B status: Delivered (Active: false)
Order C status: Cancelled (Active: false)
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Declare `enum OrderStatus`.
3. Add `impl OrderStatus` with `is_active(&self) -> bool`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_46_enums_basics
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Define the four choices and derive Debug and PartialEq.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Return the boolean expression comparing the dereferenced receiver with Pending or Shipped; the other two statuses are inactive.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

The equality operator borrows its operands, so `*self == ...` does not move this enum out of the borrowed receiver. Enums restrict choices, not legal transitions.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
