# Solution Explanation: Task 39 (String Methods)

Let's dissect how Rust provides UTF-8 text manipulation methods; some borrow existing bytes and others allocate a new String!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== SECURITY BROADCAST SANITIZER ===");

    let raw_input = "   ALERT: critical system warning!   ";
    println!("Raw Length:     {} bytes", raw_input.len());

    let clean = raw_input.trim();
    println!("Trimmed:        '{}'", clean);
    println!("Trimmed Length: {} bytes", clean.len());

    let is_critical = clean.contains("critical");
    println!("Has 'critical'? {}", is_critical);

    let replaced = clean.replace("warning", "EMERGENCY 🚨");
    println!("Replaced:       '{}'", replaced);

    let upper = clean.to_uppercase();
    let final_notice = format!("BROADCAST: {}", upper);
    println!("Final Notice:   {}", final_notice);

    println!("====================================");
}
```

---

## 🧩 Key Methods Breakdown

1. **`.trim()`:**
   * Returns a sub-slice `&str` pointing inside the original text without allocating any new heap memory!
   * The start pointer is moved past leading whitespace, and the slice length is shortened to exclude trailing whitespace.
2. **`.replace()`:**
   * Allocates and returns a brand-new `String` where matching occurrences are substituted.
3. **`format!(...)`:**
   * Behaves like `println!`, but instead of dumping bytes to stdout, builds a newly allocated, owned `String`.

## Everyday picture and symbols

Think of a word processor that returns edited text while retaining the original. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `.trim()` | Borrow text without surrounding whitespace |
| `.len()` | UTF-8 byte length |
| `.replace(a, b)` | Build a new String |
| `format!(...)` | Build formatted owned text |

## Common pitfall

Trimming and replacing text is not security sanitization for HTML, SQL, or shell commands. This example only demonstrates string methods.

## Expected output

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
