# Task 39: Security Broadcast Sanitizer (String Methods)

## 🎯 What is Your Task?
Write a Rust program to clean, analyze, and format security alert messages:
1. Start with the raw string:
   ```rust
   let raw_input = "   ALERT: critical system warning!   ";
   ```
2. Trim leading and trailing whitespace using `.trim()`.
3. Check if the message contains `"critical"` using `.contains("critical")` (print boolean result).
4. Replace `"warning"` with `"EMERGENCY 🚨"` using `.replace()`.
5. Convert the trimmed message to full uppercase using `.to_uppercase()`.
6. Use the `format!` macro to generate a final announcement string:
   `"BROADCAST: <sanitized_uppercase_text>"`

---

## 📋 Expected Output on Your Screen
```text
=== SECURITY BROADCAST SANITIZER ===
Raw Length:     37 bytes
Trimmed:        'ALERT: critical system warning!'
Trimmed Length: 31 bytes
Has 'critical'? true
Replaced:       'ALERT: critical system EMERGENCY 🚨!'
Final Notice:   BROADCAST: ALERT: CRITICAL SYSTEM WARNING!
====================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_39_string_methods/src/main.rs).
2. Use `.trim()`, `.len()`, `.contains()`, `.replace()`, `.to_uppercase()`, and `format!()`.
3. Test:
   ```bash
   cd tasks/04_collections/task_39_string_methods
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_39_string_methods/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_39_string_methods/solution/explain.md)

