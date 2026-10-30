# Task 60: The Sentinel Comparison Gate (Lifetimes 'a)

## 🎯 What is Your Task?
Write a Rust program to compare and return the longer of two string references using explicit lifetime annotations:
1. Define a function:
   ```rust
   fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
   ```
   - Checks `if x.len() >= y.len() { x } else { y }`.
2. In `main()`:
   - Declare `let sentence = String::from("The Rust Programming Language");`
   - Declare `let word = "Rust";`
   - Call `let result = longest(sentence.as_str(), word);`
   - Print `"Longest text: '<result>'"`
   - Declare a `'static` reference:
     ```rust
     let banner: &'static str = "SAFE SYSTEMS WITH ZERO DANGLING POINTERS";
     ```
   - Print the static banner!

---

## 📋 Expected Output on Your Screen
```text
=== LIFETIME COMPARISON GATE ===
Longest text: 'The Rust Programming Language'
Security Banner: SAFE SYSTEMS WITH ZERO DANGLING POINTERS ('static)
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_60_lifetimes_intro/src/main.rs).
2. Declare generic lifetime `<'a>` in function signature.
3. Compare string slice lengths and return the longer reference.
4. Test:
   ```bash
   cd tasks/06_ownership_and_memory/task_60_lifetimes_intro
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_60_lifetimes_intro/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_60_lifetimes_intro/solution/explain.md)

