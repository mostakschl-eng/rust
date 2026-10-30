# Task 16: Solution Breakdown & Explicit Casting

## 1. Line-by-Line Breakdown
* If we wrote `total_score / total_subjects` without `as f64`:
  `345 / 4` would produce `86` (losing `.25%` entirely due to integer truncation).
* By casting `(total_score as f64) / (total_subjects as f64)`, the CPU switches into floating-point mode and accurately computes `86.25`.
