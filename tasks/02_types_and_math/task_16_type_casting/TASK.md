# Task 16: Exam GPA & Score Averaging (Type Casting with `as`)

## 🎯 What is Your Task?
Write a Rust program that calculates the exact decimal average of 4 exam scores stored as integers.

Scores (Integers):
* `math: i32` = `85`
* `science: i32` = `92`
* `history: i32` = `78`
* `english: i32` = `90`

Steps:
1. Calculate `total_score = math + science + history + english`
2. Set `total_subjects: i32 = 4`
3. Cast both to `f64` using `as f64` and calculate:
   `average: f64 = (total_score as f64) / (total_subjects as f64)`
4. Print the score breakdown and the decimal average!

---

## 📋 Expected Output on Your Screen
```text
=== STUDENT SEMESTER REPORT ===
Math:           85
Science:        92
History:        78
English:        90
Total Score:    345
Subjects:       4
Exact Average:  86.25%
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_16_type_casting/src/main.rs).
2. Calculate the sum of the integer scores.
3. Cast to `as f64` before dividing.
4. Print the report.
5. Test with:
   ```bash
   cd tasks/02_types_and_math/task_16_type_casting
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Explicit casting</b></summary>

```rust
let total_score = math + science + history + english;
let average = (total_score as f64) / 4.0;
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_16_type_casting/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_16_type_casting/solution/explain.md)

