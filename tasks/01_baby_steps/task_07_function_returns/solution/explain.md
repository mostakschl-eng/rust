# Task 07: Solution Breakdown & Expressions

## 1. Line-by-Line Breakdown

```rust
fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}
```
* The return type `-> i32` promises that the output is a 32-bit whole number.
* `width * height` has no semicolon, so its result is handed back as the return value.

```rust
fn calculate_perimeter(width: i32, height: i32) -> i32 {
    2 * (width + height)
}
```
* Standard math order of operations: Parentheses `(width + height)` are evaluated first, then multiplied by `2`.

---

## 2. Using Returned Values
```rust
let area = calculate_area(width, height);
```
Here, `calculate_area` returns `50`. Rust places `50` into the variable box named `area`. Now you can reuse `area` anywhere in your code!
