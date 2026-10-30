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
    println!("First recorded score: {} pts", high_scores[0]);
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
   * Vectors store their dynamic payload on the heap, allowing them to expand whenever needed, unlike arrays whose element count is fixed.

## Everyday picture and symbols

Think of a shopping cart whose item count can change. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `Vec<i32>` | Vector containing i32 elements |
| `Vec::new()` | Create an empty vector |
| `vec![...]` | Create a vector with initial elements |
| `.push(x)` | Append an element |

## Common pitfall

A vector preserves insertion order; index 0 is 1250, not the highest score (3400). It does not sort itself.

## Expected output

```text
=== ARCADE LEADERBOARD ===
Total scores recorded: 3
First recorded score: 1250 pts
All scores: [1250, 3400, 2950]

=== PROGRAMMING LANGUAGES ===
Favorite language: Rust
All languages: ["Rust", "Python", "TypeScript"]
=============================
```
