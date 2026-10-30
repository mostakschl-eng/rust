# Task 21: Solution Breakdown & Conditional Verification

## 1. Line-by-Line Breakdown
* `account_balance >= withdraw_amount`: The greater-than-or-equal operator `>=` checks if 500 is at least 200. Since it is, the expression evaluates to `true`.
* Because it is `true`, Rust executes the block `{ ... }`.
* If `withdraw_amount` was set to `700`, the condition would evaluate to `false`. Rust would skip lines 8 to 12 entirely, preventing the ATM from dispensing money the user doesn't have!

