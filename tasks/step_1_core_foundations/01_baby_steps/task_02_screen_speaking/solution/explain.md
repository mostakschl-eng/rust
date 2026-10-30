# Task 02: Solution Breakdown & Explanations

## 1. Line-by-Line Code Explanation

```rust
fn main() {
    println!("Hello, my name is {} and I live in {}.", "Rahim", "Dhaka");
    println!("My lucky number is {} and my favorite language is {}!", 7, "Rust");
}
```

* **Line 1 (`println!("Hello, my name is {} and I live in {}.", "Rahim", "Dhaka");`):**
  - First `{}` matches the first value after the comma: `"Rahim"`.
  - Second `{}` matches the second value after the comma: `"Dhaka"`.
  - Notice the comma `,` separating the string from `"Rahim"`, and the comma separating `"Rahim"` from `"Dhaka"`.

* **Line 2 (`println!("My lucky number is {} and my favorite language is {}!", 7, "Rust");`):**
  - First `{}` matches the number `7`. Numbers do not need quotation marks.
  - Second `{}` matches `"Rust"`.

---

## 2. Common Mistakes to Watch Out For

1. **Mismatching Count:** If you have 3 `{}` placeholders but only give 2 values, the Rust compiler immediately stops you with an error. Always ensure count of `{}` equals count of values!
2. **Missing Commas:** Forgetting the comma between values (e.g. `println!("Score: {}", 100 200)`) will cause a syntax error.

