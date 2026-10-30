# Task 28: Solution Breakdown & Loop Condition Checks

## 1. Line-by-Line Breakdown
* The condition `attempts_used < max_allowed` is evaluated before every lap.
* Lap 1: 0 < 3 (true). `attempts_used` becomes 1.
* Lap 2: 1 < 3 (true). `attempts_used` becomes 2.
* Lap 3: 2 < 3 (true). `attempts_used` becomes 3.
* Next check: 3 < 3 (false!). Rust immediately stops the loop and jumps to line 12 to announce the security lockout!

