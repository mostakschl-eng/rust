# Task 13: Solution Breakdown & Boolean Logic

## 1. Line-by-Line Breakdown
* `let is_email_verified: bool = true;`: Booleans occupy exactly 1 byte in memory.
* `let can_make_purchases: bool = !is_account_suspended;`: The `!` (NOT) operator inverts the boolean. Since `is_account_suspended` is `false`, `!false` evaluates to `true`.
* If an account is suspended (`true`), `!true` automatically flips `can_make_purchases` to `false`. This is the clean, idiomatic way to write state flags in Rust!

