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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_32_tuple_destructuring/src/main.rs).
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
   cd tasks/04_collections/task_32_tuple_destructuring
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_32_tuple_destructuring/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_32_tuple_destructuring/solution/explain.md)

