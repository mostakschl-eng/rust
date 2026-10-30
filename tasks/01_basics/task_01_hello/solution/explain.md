# Task 01: Solution Breakdown & Common Beginner Mistakes

## 1. Line-by-Line Explanation

```rust
fn main() {
    println!("Hello, I am Rahim!");
    println!("I am starting my Rust developer journey!");
}
```

* **`fn main() {`**
  This defines the main function. Rust always starts executing your code here. Without `main()`, Rust doesn't know where to begin.
* **`println!("Hello, I am Rahim!");`**
  Prints the first line of text and then moves the cursor to the next line. Notice the semicolon `;` at the end.
* **`println!("I am starting my Rust developer journey!");`**
  Prints the second line of text.
* **`}`**
  The closing curly brace closes the room and tells Rust the program is finished.

---

## 2. Common Beginner Pitfalls & How to Avoid Them

### ❌ Mistake 1: Using Single Quotes instead of Double Quotes
```rust
// WRONG:
println!('Hello, World!');

// CORRECT:
println!("Hello, World!");
```
* **Why:** In Rust, single quotes `'A'` are only for a **single character**. For words and sentences (strings), you must always use double quotes `"..."`.

---

### ❌ Mistake 2: Forgetting the Semicolon `;`
```rust
// WRONG:
println!("Hello, World!")

// CORRECT:
println!("Hello, World!");
```
* **Why:** The semicolon `;` tells Rust that this instruction has ended. If you forget it, the compiler will say: `error: expected ';', found ...`.

---

### ❌ Mistake 3: Forgetting the Exclamation Mark `!`
```rust
// WRONG:
printline("Hello, World!");

// CORRECT:
println!("Hello, World!");
```
* **Why:** `println!` requires the `!` because it is a Rust macro.
