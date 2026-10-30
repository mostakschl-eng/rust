# Solution Explanation: Task 73 (Trait Basics)

Let's dissect how traits define shared capabilities without class inheritance!

---

## 🔍 The Solution Code

```rust
pub trait Summarizable {
    fn summarize(&self) -> String;
}

impl Summarizable for NewsArticle { ... }
impl Summarizable for SocialPost { ... }

fn publish_summary(item: &impl Summarizable) {
    println!("[FEED DISPATCH] {}", item.summarize());
}
```

---

## 🧩 Key Takeaways

1. **`impl Trait for Type`:**
   * Attaches the trait's contract to a specific concrete struct.
2. **`&impl Summarizable`:**
   * Syntactic sugar for a generic function with a trait bound: `fn publish_summary<T: Summarizable>(item: &T)`.
   * Accepts any type that fulfills the contract!

