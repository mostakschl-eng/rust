# Solution Explanation: Task 67 (Terminal Input)

Let's dissect how raw user keystrokes are cleaned, parsed, and converted safely!

---

## 🔍 The Solution Code

```rust
fn parse_terminal_input(raw: &str) -> (String, u32) {
    let clean = raw.trim();
    let parts: Vec<&str> = clean.split_whitespace().collect();

    let name = parts.get(0).copied().unwrap_or("Guest").to_string();
    let age = parts.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(18);

    (name, age)
}
```

---

## 🧩 Key Takeaways

1. **`.trim()` is Mandatory:**
   * Removes `\r\n` line endings produced by keyboards when users strike Enter.
2. **Safe Parsing with Fallbacks:**
   * `.parse::<u32>().ok()` transforms `Result<u32, ParseIntError>` into `Option<u32>`.
   * `.unwrap_or(18)` ensures the application continues smoothly with a default value even if the user makes a typing error!

