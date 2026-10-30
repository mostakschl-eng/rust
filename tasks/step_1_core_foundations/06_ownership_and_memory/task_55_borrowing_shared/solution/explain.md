# Solution Explanation: Task 55 (Shared References)

Let's dissect how shared borrowing gives high performance with zero memory duplication!

---

## 🔍 The Solution Code

```rust
fn analyze_text(text: &String) -> (usize, usize) {
    (text.len(), text.split_whitespace().count())
}

fn main() {
    let manifesto = String::from("Rust empowers everyone to build reliable and efficient software.");

    let (bytes, words) = analyze_text(&manifesto);

    println!("Total Bytes: {}", bytes);
    println!("Total Words: {}", words);
    println!("Original Text (Still Alive!):\n\"{}\"", manifesto);

    let r1 = &manifesto;
    let r2 = &manifesto;

    println!("Reader 1: {}", r1);
    println!("Reader 2: {}", r2);
}
```

---

## 🧩 Key Takeaways

1. **Borrowing Without Moving:**
   * `&manifesto` passes a read-only reference.
   * `manifesto` is never moved, so it remains accessible in `main` for subsequent prints.
2. **Unlimited Shared Readers:**
   * Having both `r1` and `r2` pointing to `manifesto` at the same time is completely legal and safe because neither has permission to mutate the underlying data.


## Reading the Result Carefully

Several library visitors can read one displayed page without taking ownership of the library book.

| Symbol or term | Meaning |
|---|---|
| `&` | shared borrow |
| `&str` | borrowed UTF-8 text |
| `usize` | length/count type |
| `(a, b)` | tuple |

String length counts bytes, not letters. The manifesto contains 64 ASCII bytes and 9 whitespace-separated words.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
