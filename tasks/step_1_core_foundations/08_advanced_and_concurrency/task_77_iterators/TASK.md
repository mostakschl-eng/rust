# Task 77: Gamer Score Pipeline (`filter`, `map`, `collect`, `sum`)

## 🎯 What is Your Task?
Process a vector of gaming tournament scores using an idiomatic Rust iterator pipeline:

1. In `main()`, start with:
   ```rust
   let scores = vec![45, 92, 12, 85, 33, 76, 58, 99, 64, 88];
   ```
2. Build an iterator pipeline:
   - Use `.iter()` to borrow the scores, because you must print the original vector afterward.
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

1. Open [`src/main.rs`](src/main.rs).
2. Chain `.filter(|&&score| score >= 70)` and `.map(|score| score + 5)`.
3. Collect the result using `.collect::<Vec<i32>>()`.
4. Run:
   ```bash
   cd tasks/step_1_core_foundations/08_advanced_and_concurrency/task_77_iterators
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Borrow scores with iter() because the task prints the original vector afterward.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Use filter(|&&s| s >= 70), map(|&s| s + 5), collect into Vec<i32>, then sum the resulting elements.

</details>

**Before running:** With iter(), filter sees &&i32 while map sees &i32. into_iter on an owned Vec moves the vector, so it cannot be printed afterward. iterators can also be driven one item at a time by next().
