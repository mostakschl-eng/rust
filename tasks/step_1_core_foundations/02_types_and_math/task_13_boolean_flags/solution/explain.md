# Task 13: Solution Breakdown & Boolean Logic

## 1. Line-by-Line Breakdown
* `let is_email_verified: bool = true;`: Booleans occupy exactly 1 byte in memory.
* `let can_make_purchases: bool = !is_account_suspended;`: The `!` (NOT) operator inverts the boolean. Since `is_account_suspended` is `false`, `!false` evaluates to `true`.
* If the source flag is `true` when this expression is evaluated, the result is `false`. This is a snapshot: changing the source later does not automatically recompute the derived variable.

