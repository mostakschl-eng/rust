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
   - Create `order_b = OrderStatus::Delivered`.
   - Print their statuses and whether each order is currently active.

---

## 📋 Expected Output on Your Screen
```text
=== E-COMMERCE ORDER TRACKER ===
Order A status: Pending (Active: true)
Order B status: Delivered (Active: false)
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_46_enums_basics/src/main.rs).
2. Declare `enum OrderStatus`.
3. Add `impl OrderStatus` with `is_active(&self) -> bool`.
4. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_46_enums_basics
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_46_enums_basics/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_46_enums_basics/solution/explain.md)

