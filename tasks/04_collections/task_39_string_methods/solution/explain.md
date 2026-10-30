# Solution Explanation: Task 39 (String Methods)

Let's dissect how Rust provides zero-overhead, Unicode-safe text manipulation methods!

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
   * The start pointer is moved past leading whitespace, and the end pointer is moved before trailing whitespace.
2. **`.replace()`:**
   * Allocates and returns a brand-new `String` where matching occurrences are substituted.
3. **`format!(...)`:**
   * Behaves like `println!`, but instead of dumping bytes to stdout, builds a newly allocated, owned `String`.

