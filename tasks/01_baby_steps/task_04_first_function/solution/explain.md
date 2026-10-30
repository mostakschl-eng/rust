# Task 04: Solution Breakdown & Function Concepts

## 1. Line-by-Line Code Breakdown

```rust
fn cheer() {
    println!("Rust is fast!");
    println!("Rust is safe!");
}
```
* `fn cheer()` defines our custom recipe.
* Notice that Rust does **not** run `cheer()` immediately when starting up. Rust always starts running at `fn main()`.
* When Rust is inside `main()` and encounters `cheer();`:
  1. Rust pauses execution in `main()`.
  2. It jumps into the `cheer()` function and runs both `println!` lines.
  3. When it reaches the closing `}` of `cheer()`, it jumps right back to `main()` where it left off and continues to the next line!

---

## 2. Order of Functions in the File
In Rust, it does **not** matter whether `fn cheer()` is written *above* `main()` or *below* `main()`. Rust reads the whole file first, finds all functions, and links them automatically!

