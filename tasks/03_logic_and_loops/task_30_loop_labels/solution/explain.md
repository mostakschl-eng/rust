# Solution Explanation: Task 30 (Loop Labels & Multi-Level Breaks)

Let's dissect how loop labels solve the classic "nested loop escape" dilemma in Rust!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== MINEFIELD RADAR SCANNER ===");

    'radar_grid: for row in 1..=4 {
        for col in 1..=4 {
            println!("Radar scanning [Row {}, Col {}]...", row, col);

            if row == 3 && col == 2 {
                println!("⚠️ ALERT: Secret Treasure Detected at [3, 2]!");
                println!("Initiating emergency radar shutdown!");
                break 'radar_grid;
            }
        }
    }

    println!("Radar deactivated. Mission successful! 🏁");
    println!("===============================");
}
```

---

## 🧩 Line-by-Line Breakdown

### 1. The Loop Label: `'radar_grid:`
```rust
'radar_grid: for row in 1..=4 {
```
* **Single Quote (`'`)**: In Rust, all loop labels (and lifetimes!) start with a single quote.
* **Name**: Any identifier, e.g. `'outer`, `'scan`, `'radar_grid`.
* **Colon (`:`)**: Follows the label name immediately before `for`, `while`, or `loop`.
* **Meaning**: Gives the outer loop a unique name badge so we can refer to it from anywhere inside its body—even 10 levels deep!

### 2. The Inner Loop: `for col in 1..=4`
```rust
for col in 1..=4 {
```
* This runs 4 times for **every single** row iteration.
* When `row = 1`, `col` goes `1, 2, 3, 4`.
* When `row = 2`, `col` goes `1, 2, 3, 4`.
* When `row = 3`, `col = 1` then `col = 2`.

### 3. Target Detection Condition:
```rust
if row == 3 && col == 2 {
```
* `&&` means **AND**—both conditions must be true simultaneously.

### 4. The Multi-Level Escape:
```rust
break 'radar_grid;
```
* Without the label, a plain `break;` would **only** stop the inner `col` loop. The outer `row` loop would keep running for rows 3 and 4!
* With `break 'radar_grid;`, Rust jumps out of **BOTH** loops straight to the line right after `'radar_grid:` closes!
* No messy boolean flags like `let mut found = false;` needed!

---

## 🧪 Output Verification

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
Row 3 Col 3, Col 4, and Row 4 were never scanned because the labeled break aborted the entire grid scan instantly! 🎯

