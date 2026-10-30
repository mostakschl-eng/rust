# Task 32: Grade Analytics & Tuple Destructuring

## 🎯 What is Your Task?
Write a Rust program with a helper function `calculate_stats` that takes three exam scores and returns a tuple containing:
1. Total Sum (`i32`)
2. Average (`f64`)
3. Highest Score (`i32`)

In `main()`:
1. Call `calculate_stats(85, 92, 78)`.
2. Destructure the returned tuple into `(total, average, highest)`.
3. Print each stat cleanly.
4. Call `calculate_stats(90, 95, 100)` and destructure using `_` to extract **only the highest score**, ignoring the sum and average!

---

## 📋 Expected Output on Your Screen
```text
=== EXAM BATCH 1 RESULTS ===
Total Points:  255
Average Score: 85.00
Highest Score: 92

=== EXAM BATCH 2 RESULTS ===
Batch 2 Top Score: 100
============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write function:
   ```rust
   fn calculate_stats(s1: i32, s2: i32, s3: i32) -> (i32, f64, i32) {
       let sum = s1 + s2 + s3;
       let avg = sum as f64 / 3.0;
       let mut max = s1;
       if s2 > max { max = s2; }
       if s3 > max { max = s3; }
       (sum, avg, max)
   }
   ```
3. Destructure batch 1:
   ```rust
   let (total, average, highest) = calculate_stats(85, 92, 78);
   ```
4. Destructure batch 2 with `_`:
   ```rust
   let (_, _, top_score) = calculate_stats(90, 95, 100);
   ```
5. Test:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_32_tuple_destructuring
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Compute the three statistics first; the function returns one tuple.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Destructure the first result by name; in the second pattern use `_` in positions zero and one.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

Divide a floating-point sum by `3.0`; integer division would lose the fraction. Scores in this exercise are 0–100, so their sum fits in i32.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.

## Input range

Use exam scores from 0 to 100. Their total fits in i32. The helper is not intended to sum arbitrary extreme i32 values.
