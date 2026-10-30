# Task 62: Game Score Lookup Service (Option Enum)

## 🎯 What is Your Task?
Write a Rust program with an optional score lookup service:
1. Define a function `find_player_score(player_id: u32) -> Option<u32>`:
   - Returns `Some(950)` if `player_id == 101`.
   - Returns `Some(820)` if `player_id == 102`.
   - Returns `None` for all other unknown IDs.
2. In `main()`:
   - Look up Player 101: use `.is_some()` to verify presence, then unwrap with `match` and print `"Player 101 Score: 950 pts"`.
   - Look up Player 999: verify with `match` and print `"Player 999: No record found."`.

---

## 📋 Expected Output on Your Screen
```text
=== GAME SCORE LOOKUP SERVICE ===
Querying Player #101:
Found record? true
Player 101 Score: 950 pts

Querying Player #999:
Player 999: No record found.
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_62_option_enum/src/main.rs).
2. Implement `find_player_score`.
3. Test:
   ```bash
   cd tasks/07_errors_and_io/task_62_option_enum
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_62_option_enum/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_62_option_enum/solution/explain.md)

