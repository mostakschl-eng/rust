# Task 09: Arcade Score Tracker (Mutability with `mut`)

## 🎯 What is Your Task?
Write a Rust program simulating an **Arcade Game Score Tracker** using a mutable variable (`let mut`).

Follow these exact game rounds:
1. Start the game with `score = 0`.
2. Print `"Game Started! Score: 0"`.
3. Round 1: Player defeats a monster! Add `150` points to `score` and print the updated score.
4. Round 2: Player finds a secret treasure! Add `300` points to `score` and print the updated score.
5. Round 3: Player touches a trap! Subtract `50` points from `score` and print the final score.

---

## 📋 Expected Output on Your Screen
```text
=== ARCADE LEADERBOARD ===
Game Started! Starting Score: 0
Round 1: Defeated monster (+150) -> Score: 150
Round 2: Secret treasure  (+300) -> Score: 450
Round 3: Trap penalty     (-50)  -> Score: 400
=== FINAL SCORE: 400 POINTS ===
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_09_mutability_score/src/main.rs).
2. Declare: `let mut score = 0;`.
3. In each round, update `score = score + 150;` (or `score += 150;`).
4. Print the score after each round.
5. Test with:
   ```bash
   cd tasks/01_baby_steps/task_09_mutability_score
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Adding and subtracting from mutable variable</b></summary>

```rust
let mut score = 0;
score = score + 150;
score = score + 300;
score = score - 50;
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_09_mutability_score/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_09_mutability_score/solution/explain.md)

