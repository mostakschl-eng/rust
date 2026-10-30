# Task 27: Solution Breakdown & Loop Value Extraction

## 1. Line-by-Line Breakdown
* The loop starts with `balance = 200`.
* Year 1: $350
* Year 2: $500
* Year 3: $650
* Year 4: $800
* Year 5: $950 (Still < 1000)
* Year 6: $1100 (>= 1000 condition satisfied!)
* `break years;` passes the number `6` directly to `years_taken`.
* The variable `years_taken` is completely immutable, providing safety and avoiding mutable bugs later in the program!

