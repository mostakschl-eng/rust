# Task 67: Terminal User Greeter (Standard Input std::io::stdin)

## 🎯 What is Your Task?
Write a Rust program to clean, sanitize, and parse raw terminal text input:
1. Define a helper function `parse_terminal_input(raw: &str) -> (String, u32)`:
   - Trims surrounding whitespace and newlines with `.trim()`.
   - Splits on whitespace into parts.
   - Returns the name and parsed age (or defaults to `18` if missing/invalid).
2. In `main()`:
   - Process raw simulated input: `"  Tanvir 22\r\n"`.
   - Print clean name and age.
   - Demonstrate standard `io::stdin().read_line(&mut buffer)` interactive readiness.

---

## 📋 Expected Output on Your Screen
```text
=== TERMINAL INPUT SANITIZER ===
Raw Input:       "  Tanvir 22\r\n"
Sanitized Name:  Tanvir
Parsed Age:      22 years old
Terminal reader initialized and verified! ⌨️
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_67_terminal_user_input/src/main.rs).
2. Clean raw text using `.trim()`.
3. Test:
   ```bash
   cd tasks/07_errors_and_io/task_67_terminal_user_input
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_67_terminal_user_input/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_67_terminal_user_input/solution/explain.md)

