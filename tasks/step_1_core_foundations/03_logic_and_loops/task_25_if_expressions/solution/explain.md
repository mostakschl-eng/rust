# Task 25: Solution Breakdown & Expression Value Assignment

## 1. Line-by-Line Breakdown
* In Rust, `if/else` is an expression. Both `0.00` and `7.99` evaluate to `f64`.
* Because `order_total >= 50.0` is `true`, `0.00` is returned and stored directly into the immutable variable `shipping_fee`.
* `shipping_fee` is initialized once, so it does not need `mut`. Delayed initialization in both branches is also valid Rust (see Task 22).


## Precision note

This is a small arithmetic/formatting exercise. Binary floats approximate many decimal amounts; display formatting does not make the stored number exact. Real money calculations need a defined rounding policy and an appropriate representation, such as integer minor units.
