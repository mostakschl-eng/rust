# Task 53: Function Dispatcher (Move vs Copy Semantics)

## 🎯 What is Your Task?
Write a Rust program demonstrating how function parameters handle Copy values versus non-Copy moves:
1. Define a function `copy_primitive(val: i32)`:
   - Prints `"Inside copy_primitive: <val>"`
2. Define a function `consume_string(text: String)`:
   - Prints `"Inside consume_string: <text> (Taking full ownership)"`
3. In `main()`:
   - Declare `let original_num: i32 = 500;`
   - Pass `original_num` to `copy_primitive(original_num)`.
   - Print `original_num` again in `main` to prove i32 implements `Copy` and remain valid!
   - Declare `let parcel = String::from("Rocket Fuel");`
   - Pass `parcel` to `consume_string(parcel)`.
   - Print `"Parcel ownership successfully moved and cleaned up."`

---

## 📋 Expected Output on Your Screen
```text
=== MOVE VS COPY DISPATCHER ===
[1. Testing Primitive Copy]
Inside copy_primitive: 500
Still alive in main:   500 (Copy trait preserved it!)

[2. Testing Heap Move]
Inside consume_string: Rocket Fuel (Taking full ownership)
Parcel ownership successfully moved and cleaned up.
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Implement both functions.
3. Call both functions and observe that `original_num` is reusable while `parcel` moved.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/06_ownership_and_memory/task_53_move_vs_copy
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Put the integer and String into separate helper functions with the requested parameter types.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Print the integer again after its call. The String has moved, so do not print the old binding after its call.

</details>

**Before running:** No standard Move trait exists. A non-Copy struct can contain only numbers and still move; location does not decide Copy.
