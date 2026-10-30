# Task 51: Memory Architecture Inspector (Stack vs Heap)

## 🎯 What is Your Task?
Write a Rust program demonstrating stack primitives alongside growable heap containers:
1. Declare three fixed-size stack variables:
   - `age: i32 = 25`
   - `rating: f64 = 4.9`
   - `is_pro: bool = true`
2. Declare a mutable heap-allocated `bio: String` initialized with `"Rust Explorer"`.
3. Append `" & System Architect"` to `bio` using `.push_str()`.
4. Declare a mutable heap-allocated vector `skill_levels: Vec<i32>` initialized with `vec![85, 90, 95]`.
5. Append `99` to `skill_levels` using `.push(99)`.
6. Print both stack primitives and the dynamically expanded heap containers.

---

## 📋 Expected Output on Your Screen
```text
=== MEMORY ARCHITECTURE INSPECTOR ===
[STACK MEMORY]
Age:     25 years
Rating:  4.90 / 5.0
Pro Dev: true

[HEAP MEMORY]
Bio:     Rust Explorer & System Architect
Skills:  [85, 90, 95, 99] (Count: 4)
=====================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_51_stack_vs_heap/src/main.rs).
2. Declare stack primitives and heap objects.
3. Mutate heap objects using `.push_str()` and `.push()`.
4. Test:
   ```bash
   cd tasks/06_ownership_and_memory/task_51_stack_vs_heap
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_51_stack_vs_heap/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_51_stack_vs_heap/solution/explain.md)

