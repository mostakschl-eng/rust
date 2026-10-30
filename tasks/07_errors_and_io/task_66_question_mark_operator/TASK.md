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
   - Print both results using `{:?}`.

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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_66_question_mark_operator/src/main.rs).
2. Write `calculate_volume` using `?` for early error propagation.
3. Test:
   ```bash
   cd tasks/07_errors_and_io/task_66_question_mark_operator
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_66_question_mark_operator/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_66_question_mark_operator/solution/explain.md)

