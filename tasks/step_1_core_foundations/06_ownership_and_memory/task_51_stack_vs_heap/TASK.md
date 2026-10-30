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

1. Open [`src/main.rs`](src/main.rs).
2. Declare stack primitives and heap objects.
3. Mutate heap objects using `.push_str()` and `.push()`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/06_ownership_and_memory/task_51_stack_vs_heap
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Separate fixed-size bindings from growable buffers before printing.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Use `mut` for each container you grow; `.push_str()` adds text and `.push()` adds one vector element.

</details>

**Before running:** A type is not inherently a stack type or heap type. Local storage may be optimized into registers, and a fixed-size array can be boxed.
