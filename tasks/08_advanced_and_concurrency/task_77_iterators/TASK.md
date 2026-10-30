# Task 77: Gamer Score Pipeline (`filter`, `map`, `collect`, `sum`)

## 🎯 What is Your Task?
Process an array of gaming tournament scores using an idiomatic Rust Iterator pipeline:

1. In `main()`, start with:
   ```rust
   let scores = vec![45, 92, 12, 85, 33, 76, 58, 99, 64, 88];
   ```
2. Build an iterator pipeline:
   - Use `.iter()` or `.into_iter()` (use `.iter()` if you want to keep `scores` around for printing).
   - `.filter()` to keep only scores that are `>= 70`.
   - `.map()` to award a `+5` bonus to each qualifying score.
   - `.collect()` into a `Vec<i32>` named `honors_scores`.
3. Calculate the sum of `honors_scores` using `.iter().sum::<i32>()`.
4. Print the original scores, the boosted honors scores, and the total!

---

## 📋 Expected Output on Your Screen
```text
=== GAMER SCORE PIPELINE ===
Original Scores: [45, 92, 12, 85, 33, 76, 58, 99, 64, 88]
Honors (+5):     [97, 90, 81, 104, 93]
Total Honors:    465
============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_77_iterators/src/main.rs).
2. Chain `.filter(|&&score| score >= 70)` and `.map(|score| score + 5)`.
3. Collect the result using `.collect::<Vec<i32>>()`.
4. Run:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_77_iterators
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_77_iterators/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_77_iterators/solution/explain.md)

