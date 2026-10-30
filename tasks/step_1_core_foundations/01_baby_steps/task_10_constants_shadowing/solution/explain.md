# Task 10: Solution Breakdown & Shadowing Magic

## 1. Line-by-Line Breakdown

```rust
const TAX_PERCENTAGE: i32 = 10;
```
* `const` defines an unchangeable constant value. It has an explicit type annotation `: i32` and is written in uppercase `TAX_PERCENTAGE`.

```rust
let price = 100;
let price = price - 20;
```
* Notice that `price` is **immutable** (no `mut`).
* By writing `let price = price - 20;`, we are NOT modifying the old variable; we are creating a brand new immutable variable named `price` that shadows (hides) the old one.
* This allows step-by-step transformations with new bindings. For these chosen values, whole-dollar integer division gives exactly $8 tax. Other prices may have fractional tax; real billing needs minor units and an explicit rounding policy.

