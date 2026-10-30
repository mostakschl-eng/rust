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


## Reading the Result Carefully

A press office asks every source for a summary. An article and a social post provide different wording through the same promised method.

| Symbol or term | Meaning |
|---|---|
| `trait` | behavior contract |
| `impl Trait for Type` | fulfill it |
| `&impl Trait` | borrow any one suitable concrete type |

A trait implementation must match the declared method signature. Standard method-call syntax may require importing the trait when it lives in another module.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
