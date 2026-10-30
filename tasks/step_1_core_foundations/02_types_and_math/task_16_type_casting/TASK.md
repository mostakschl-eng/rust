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

1. Open [`src/main.rs`](src/main.rs).
2. Calculate the sum of the integer scores.
3. Cast to `as f64` before dividing.
4. Print the report.
5. Test with:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_16_type_casting
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Add the four integer scores first. Casting after integer division would leave the fraction already lost.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Convert both the total and subject count to the same float type before dividing; print the result with the other report fields.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

