# Task 29: Solution Breakdown & Range Iterators

## 1. Line-by-Line Breakdown
* `1..=10`: The inclusive range iterator yields values starting at 1, increasing by 1 on each lap, and finishing exactly when 10 has been processed.
* Inside the loop, `rep` is an immutable binding that takes the current value (`1`, `2`, ..., `10`).
* When `rep == 5`, the nested `if` statement executes the encouraging message without breaking the loop!

