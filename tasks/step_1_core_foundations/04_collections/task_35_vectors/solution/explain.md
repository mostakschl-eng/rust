# Solution Explanation: Task 35 (Vectors)

Let's dissect how dynamically allocated vectors work in Rust!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== ARCADE LEADERBOARD ===");

    let mut high_scores: Vec<i32> = Vec::new();
    high_scores.push(1250);
    high_scores.push(3400);
    high_scores.push(2950);

    println!("Total scores recorded: {}", high_scores.len());
    println!("Top score: {} pts", high_scores[0]);
    println!("All scores: {:?}", high_scores);

    println!("\n=== PROGRAMMING LANGUAGES ===");

    let languages = vec!["Rust", "Python", "TypeScript"];

    println!("Favorite language: {}", languages[0]);
    println!("All languages: {:?}", languages);

    println!("=============================");
}
```

---

## 🧩 Key Takeaways

1. **`Vec::new()`:**
   * Starts with length 0 and capacity 0.
   * As you call `.push()`, Rust allocates heap memory automatically.
   * Requires `mut` because adding elements modifies the internal pointer and size!
2. **`vec![]` Macro:**
   * Shorthand to allocate and populate a vector with known items upfront.
   * `vec!["Rust", "Python", "TypeScript"]` infers the type `Vec<&str>`.
3. **Heap vs Stack:**
   * Vectors store their dynamic payload on the heap, allowing them to expand whenever needed, unlike fixed-size stack arrays.

