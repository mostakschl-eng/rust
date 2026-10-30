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

1. Open [`src/main.rs`](src/main.rs).
2. Wrap nested `List` instances in `Box::new(...)`.
3. Run:
   ```bash
   cd tasks/step_1_core_foundations/08_advanced_and_concurrency/task_78_box_smart_pointer
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Define List with Box<List> for the recursive tail, not a direct List field.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Build from the final Nil outward, wrapping each tail in Box::new; dereference the independent boxed integer to print its value.

</details>

**Before running:** Do not add field widths and assume the enum is 12 bytes. Tags, alignment, padding, target, and layout choices affect size. Sized Box handles are pointer-sized; unsized boxes also carry metadata.
