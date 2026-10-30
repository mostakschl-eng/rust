# Task 24: Solution Breakdown & Logical Grouping

## 1. Line-by-Line Breakdown
* Evaluation starts on the left: `has_ticket` is `true`, so Rust evaluates the grouped ID condition. `false || true` is `true`, allowing boarding.
* `&&` binds more tightly than `||`. Without parentheses, `has_ticket && has_passport || has_national_id` would allow anyone with a national ID even if `has_ticket` were `false`.
* Parentheses express the rule that a ticket is required with either kind of ID.

