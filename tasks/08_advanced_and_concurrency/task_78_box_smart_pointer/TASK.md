# Task 78: Smart Pointers & Recursive Linked Lists (`Box<T>`)

## 🎯 What is Your Task?
Implement heap allocation and a recursive data structure using Rust's `Box<T>` smart pointer:

1. **Heap Allocation**:
   - Allocate an integer `100` on the heap using `Box::new(100)`.
   - Read its value using dereference `*boxed_val`.

2. **Recursive Linked List**:
   - Define a recursive enum `List`:
     ```rust
     #[derive(Debug)]
     enum List {
         Cons(i32, Box<List>),
         Nil,
     }
     ```
   - In `main()`, construct a 3-element list: `10 -> 20 -> 30 -> Nil`.
   - Print the constructed list with `{:?}`.

---

## 📋 Expected Output on Your Screen
```text
=== SMART POINTERS (BOX<T>) ===
Heap Allocated Integer:  100
Constructed Linked List: Cons(10, Cons(20, Cons(30, Nil)))
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_78_box_smart_pointer/src/main.rs).
2. Wrap nested `List` instances in `Box::new(...)`.
3. Run:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_78_box_smart_pointer
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_78_box_smart_pointer/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_78_box_smart_pointer/solution/explain.md)

