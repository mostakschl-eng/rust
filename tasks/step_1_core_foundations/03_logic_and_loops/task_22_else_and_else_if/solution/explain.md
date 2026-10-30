# Task 22: Solution Breakdown & Decision Ladders

## 1. Line-by-Line Breakdown
* With `age = 68`:
  1. `68 < 5` is `false`. (Moves to next branch)
  2. `68 <= 12` is `false`. (Moves to next branch)
  3. `68 <= 64` is `false`. (Moves to next branch)
  4. Rust enters the fallback `else` branch and selects `Senior Citizen` with `price = 10`.
* In Lesson 25, we will discover how Rust lets you assign `let (category, price) = if ...` directly on a single line!


## Why no mut is needed

`let category: &str;` declares a binding without initializing it yet. Each branch gives it exactly one initial value, and every branch does so before it is printed. This is delayed initialization, not reassignment. The same reasoning applies to price.
