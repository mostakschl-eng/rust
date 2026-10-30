# Task 25: Solution Breakdown & Expression Value Assignment

## 1. Line-by-Line Breakdown
* In Rust, `if/else` is an expression. Both `0.00` and `7.99` evaluate to `f64`.
* Because `order_total >= 50.0` is `true`, `0.00` is returned and stored directly into the immutable variable `shipping_fee`.
* Notice that `shipping_fee` never had to be declared `mut`! In other languages, you must make a variable mutable or use ternary operators (`? :`). Rust's `if` expression keeps your code immutable, safe, and readable!

