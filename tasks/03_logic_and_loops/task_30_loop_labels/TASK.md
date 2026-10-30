# Task 30: Minefield Radar Scanner (Nested Loops & Loop Labels)

## 🎯 What is Your Task?
Write a Rust program simulating a **Robot Minefield Scanner** scanning a 4x4 coordinate grid using nested `for` loops and a labeled outer loop.

Grid coordinates:
* `row in 1..=4`
* `col in 1..=4`

Requirements:
1. Label the outer loop: `'radar_grid: for row in 1..=4 { ... }`
2. Inner loop: `for col in 1..=4 { ... }`
3. Print `"Radar scanning [Row {}, Col {}]..."`
4. Detect a buried treasure mine at coordinates `row == 3 && col == 2`:
   - Print `"⚠️ ALERT: Secret Treasure Detected at [3, 2]!"`
   - Print `"Initiating emergency radar shutdown!"`
   - Break out of **BOTH** loops in one single shot:
     ```rust
     break 'radar_grid;
     ```
5. Print `"Radar deactivated. Mission successful! 🏁"`

---

## 📋 Expected Output on Your Screen
```text
=== MINEFIELD RADAR SCANNER ===
Radar scanning [Row 1, Col 1]...
Radar scanning [Row 1, Col 2]...
Radar scanning [Row 1, Col 3]...
Radar scanning [Row 1, Col 4]...
Radar scanning [Row 2, Col 1]...
Radar scanning [Row 2, Col 2]...
Radar scanning [Row 2, Col 3]...
Radar scanning [Row 2, Col 4]...
Radar scanning [Row 3, Col 1]...
Radar scanning [Row 3, Col 2]...
⚠️ ALERT: Secret Treasure Detected at [3, 2]!
Initiating emergency radar shutdown!
Radar deactivated. Mission successful! 🏁
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_30_loop_labels/src/main.rs).
2. Label the outer loop with `'radar_grid:`.
3. Break using `break 'radar_grid;` when `row == 3 && col == 2`.
4. Test with:
   ```bash
   cd tasks/03_logic_and_loops/task_30_loop_labels
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Loop label syntax</b></summary>

```rust
'radar_grid: for row in 1..=4 {
    for col in 1..=4 {
        if row == 3 && col == 2 {
            break 'radar_grid;
        }
    }
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_30_loop_labels/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_30_loop_labels/solution/explain.md)

