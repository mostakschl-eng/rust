# Task 09: Solution Breakdown & Mutability

## 1. Line-by-Line Breakdown

```rust
let mut score = 0;
```
* `let mut` creates a changeable variable initialized to `0`. Without `mut`, any attempt to change `score` later will result in a compiler error.

```rust
score = score + 150;
```
* The computer reads the current value of `score` (which is `0`), adds `150`, and stores `150` back into `score`.
* In Rust, you can also write this compactly as:
  ```rust
  score += 150;
  ```
  For this integer score, both forms calculate the same new value.

---

## 2. Why Immutability Matters
Because Rust forces you to write `mut` explicitly, whenever you read someone else's Rust code, you immediately know which variables are guaranteed never to change, and which ones will be modified. This makes intended reassignment easier to see; `mut` permits changes but does not require them.

