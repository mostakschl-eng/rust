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

1. Open [`src/main.rs`](src/main.rs).
2. Use `.trim()`, `.len()`, `.contains()`, `.replace()`, `.to_uppercase()`, and `format!()`.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_39_string_methods
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Keep the trimmed view, replacement result, and uppercase result as separate values.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

The final notice uppercases `clean`, not `replaced`; replacing text returns a new String and leaves `clean` unchanged.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

Trimming and replacing text is not security sanitization for HTML, SQL, or shell commands. This example only demonstrates string methods.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
