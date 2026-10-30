# Task 03: Solution Breakdown & Comments Explanation

## 1. Line-by-Line Code Breakdown

```rust
fn main() {
    // Section 1: Print top decorative header
    println!("========================================");
    println!("       WELCOME TO MY RUST JOURNEY       ");
    println!("========================================");

    // Section 2: Print student details and goals
    println!("Student Name:  Rahim");
    println!("Goal:          Master Rust from Zero!");
    println!("Status:        Level 1 in Progress");

    // Section 3: Print bottom closing border
    println!("========================================");
}
```

* Lines starting with `//` are completely skipped by the Rust compiler. They do not execute; they remain part of the source file.
* Alignment: Notice how `Student Name: `, `Goal:         `, and `Status:       ` are padded with spaces so the colon `:` and the text line up neatly like a table!

---

## 2. Pro Tip: Using `\n` in a Single Print Call
Instead of 7 lines of `println!`, you can also do this in one single statement:
```rust
println!("========================================\n       WELCOME TO MY RUST JOURNEY       \n========================================");
```
Both ways are completely valid Rust. Multiple `println!` lines are often easier for human eyes to read!

