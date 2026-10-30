# Task 66: Shipping Cargo Dimension Calculator (? Operator)

## 🎯 What is Your Task?
Write a Rust program to calculate the volume of a 3D cargo container from text inputs using the `?` operator:
1. Define a function:
   ```rust
   fn calculate_volume(w_str: &str, h_str: &str, d_str: &str) -> Result<i32, std::num::ParseIntError>
   ```
   - Use the `?` operator on each `.parse::<i32>()` call.
   - Return `Ok(w * h * d)`.
2. In `main()`:
   - Calculate valid container: `"10"`, `"20"`, `"5"`.
   - Calculate invalid container: `"10"`, `"invalid_height"`, `"5"`.
   - Match both results; format a successful volume with its cm³ unit and the parse error using `{:?}`.

---

## 📋 Expected Output on Your Screen
```text
=== CARGO VOLUME CALCULATOR ===
Batch 1 (Valid Input):
Result: Ok(1000 cm³)

Batch 2 (Malformed Input):
Result: Err(ParseIntError { kind: InvalidDigit })
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write `calculate_volume` using `?` for early error propagation.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/07_errors_and_io/task_66_question_mark_operator
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Parse each of the three strings separately with ?.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Keep one final Ok around the product; the first invalid parse returns before later statements run.

</details>

**Before running:** The ? operator does not catch overflow or validate positive dimensions. This introductory calculator assumes small inputs whose product fits i32; real dimensions need validation and checked multiplication.
