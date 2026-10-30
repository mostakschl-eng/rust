# Task 12: Solution Breakdown & Floating-Point Explanations

## 1. Line-by-Line Breakdown
* `let price_per_gallon: f64 = 3.85;`: We declare a 64-bit float decimal.
* `let fuel_cost = gallons_pumped * price_per_gallon;`: Both variables are `f64`, so Rust performs high-precision floating-point multiplication.
* Decimal math in computers uses IEEE-754 standard representation. In Lesson 18, we will learn how to round `$45.625` to `$45.63` using formatting specifiers!

