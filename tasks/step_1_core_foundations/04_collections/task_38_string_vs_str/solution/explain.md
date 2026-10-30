# Solution Explanation: Task 38 (String vs &str)

Let's dissect how string slices (`&str`) and owned strings (`String`) cooperate in Rust!

---

## 🔍 The Solution Code

```rust
fn print_badge(role: &str, code: &str) {
    println!("[BADGE] Role: {} | Code: {}", role, code);
}

fn main() {
    println!("=== BADGE DISPATCH SYSTEM ===");

    let role_slice: &str = "SECURITY CHIEF";

    let mut badge_code = String::from("SEC");
    badge_code.push('-');
    badge_code.push_str("9042");

    print_badge(role_slice, &badge_code);

    println!("=============================");
}
```

---

## 🧩 Key Takeaways

1. **`&str` Parameters:**
   * `fn print_badge(role: &str, code: &str)`
   * Because it takes `&str`, you can pass a string literal directly (`role_slice`).
2. **Deref Coercion (`&badge_code`):**
   * `badge_code` is a heap-allocated `String`.
   * When you pass `&badge_code`, Rust automatically converts the `&String` reference into a `&str` slice for the function!
3. **Appending:**
   * `.push('-')`: for individual single-byte or multi-byte `char` literals (`' '`).
   * `.push_str("9042")`: for string slices (`" "`).

## Everyday picture and symbols

Think of borrowing a view of a notebook instead of taking the notebook. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `String` | Owned growable UTF-8 text |
| `&str` | Shared view of existing UTF-8 text |
| `&badge_code` | Borrow the owned String |
| `.push('-')` | Append one char |

## Common pitfall

Borrowing a String as &str does not copy its bytes or transfer ownership. The owner must remain alive while the view is used.

## Expected output

```text
=== BADGE DISPATCH SYSTEM ===
[BADGE] Role: SECURITY CHIEF | Code: SEC-9042
=============================
```
