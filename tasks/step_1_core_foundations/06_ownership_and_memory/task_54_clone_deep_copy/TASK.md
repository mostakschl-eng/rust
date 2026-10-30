# Task 54: Document Duplicator (Deep Copying with .clone())

## 🎯 What is Your Task?
Write a Rust program to demonstrate deep copying with `.clone()`:
1. Create a mutable `original_doc = String::from("Draft v1");`.
2. Clone `original_doc` into a new mutable variable `cloned_doc`:
   ```rust
   let mut cloned_doc = original_doc.clone();
   ```
3. Append `" (Archived Copy)"` to `original_doc`.
4. Append `" (Revised by Editor)"` to `cloned_doc`.
5. Print both documents to prove that modifying the clone did not affect the original!
6. Define a struct `TaskItem` with `#[derive(Debug, Clone)]`:
   - `title: String`
   - `priority: u8`
7. Create `task1`, clone it to `task2`, update `task2.priority = 1`, and print both!

---

## 📋 Expected Output on Your Screen
```text
=== DOCUMENT DEEP COPY ENGINE ===
Original: Draft v1 (Archived Copy)
Cloned:   Draft v1 (Revised by Editor)

=== CLONING CUSTOM STRUCT ===
Task 1: TaskItem { title: "Fix Server Crash", priority: 5 }
Task 2: TaskItem { title: "Fix Server Crash", priority: 1 }
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Use `.clone()` on `String` and on the derived struct.
3. Mutate independently and observe results.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/06_ownership_and_memory/task_54_clone_deep_copy
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Clone before adding either suffix, so both documents begin with the same draft.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Derive Clone for TaskItem, clone task1, then change only task2.priority.

</details>

**Before running:** String clones own separate text. References and Rc/Arc clones share their referent; Clone is not a universal deep-copy guarantee.
