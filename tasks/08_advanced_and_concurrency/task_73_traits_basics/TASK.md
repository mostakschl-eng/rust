# Task 73: Media Feed Feed Aggregator (Trait Basics)

## 🎯 What is Your Task?
Write a Rust program to define and implement a shared capability trait across different media types:
1. Define a trait `Summarizable`:
   ```rust
   pub trait Summarizable {
       fn summarize(&self) -> String;
   }
   ```
2. Implement `Summarizable` for:
   - `NewsArticle { headline: String, journalist: String }` -> `"<headline>" by <journalist>`
   - `SocialPost { handle: String, likes: u32 }` -> `@<handle> (Likes: <likes>)`
3. Write a helper function accepting `&impl Summarizable`:
   ```rust
   fn publish_summary(item: &impl Summarizable)
   ```
4. In `main()`, instantiate both types and print their summaries through `publish_summary`!

---

## 📋 Expected Output on Your Screen
```text
=== MEDIA FEED AGGREGATOR ===
[FEED DISPATCH] 'Rust Conquers Systems Programming' by Maya Chen
[FEED DISPATCH] @rustacean_daily (Likes: 1250)
=============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_73_traits_basics/src/main.rs).
2. Declare `trait Summarizable` and `impl Summarizable for ...`.
3. Test:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_73_traits_basics
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_73_traits_basics/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_73_traits_basics/solution/explain.md)

