# Task 35: Dynamic Leaderboard & Favorites (Vectors)

## 🎯 What is Your Task?
Write a Rust program to manage dynamic lists using both `Vec::new()` and `vec![]`:
1. Create a mutable vector `high_scores` using `Vec::new()`.
2. Push three integer scores into it: `1250`, `3400`, `2950`.
3. Create a second vector `languages` using the `vec![]` macro with: `"Rust"`, `"Python"`, `"TypeScript"`.
4. Print:
   - Leaderboard entries count and the top score at index 0.
   - The full `high_scores` vector using `{:?}`.
   - Favorite language at index 0.
   - The full `languages` vector using `{:?}`.

---

## 📋 Expected Output on Your Screen
```text
=== ARCADE LEADERBOARD ===
Total scores recorded: 3
Top score: 1250 pts
All scores: [1250, 3400, 2950]

=== PROGRAMMING LANGUAGES ===
Favorite language: Rust
All languages: ["Rust", "Python", "TypeScript"]
=============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_35_vectors/src/main.rs).
2. Create `let mut high_scores: Vec<i32> = Vec::new();`.
3. Call `.push(1250)`, `.push(3400)`, `.push(2950)`.
4. Create `let languages = vec!["Rust", "Python", "TypeScript"];`.
5. Test:
   ```bash
   cd tasks/04_collections/task_35_vectors
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_35_vectors/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_35_vectors/solution/explain.md)

