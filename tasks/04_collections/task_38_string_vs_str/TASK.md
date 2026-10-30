# Task 38: Security Badge Dispatcher (String vs &str)

## 🎯 What is Your Task?
Write a Rust program demonstrating the conversion and passing of text between `&str` and `String`:
1. Define a helper function `print_badge(role: &str, code: &str)` that accepts string slices and prints:
   `"[BADGE] Role: <role> | Code: <code>"`
2. In `main()`:
   - Declare a fixed string slice literal: `let role_slice: &str = "SECURITY CHIEF";`.
   - Create an owned `String` from `"SEC"`: `let mut badge_code = String::from("SEC");`.
   - Append character `'-'` using `.push('-')`.
   - Append text `"9042"` using `.push_str("9042")`.
   - Call `print_badge(role_slice, &badge_code)`. Notice how `&badge_code` automatically converts from `&String` to `&str`!

---

## 📋 Expected Output on Your Screen
```text
=== BADGE DISPATCH SYSTEM ===
[BADGE] Role: SECURITY CHIEF | Code: SEC-9042
=============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_38_string_vs_str/src/main.rs).
2. Implement `print_badge(role: &str, code: &str)`.
3. In `main()`, create the `&str` and owned `String`, mutate the `String`, and pass `&badge_code`.
4. Test:
   ```bash
   cd tasks/04_collections/task_38_string_vs_str
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_38_string_vs_str/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_38_string_vs_str/solution/explain.md)

