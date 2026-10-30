# Solution Explanation: Task 34 (Array Indexing & Bounds Checking)

Let's dissect how Rust provides bounds checks to prevent out-of-bounds memory access!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== SERVER STATUS CODE LOOKUP ===");

    let codes = [101, 204, 301, 404, 500];

    println!("Direct check [Index 1]: {}", codes[1]);
    println!("Direct check [Index 3]: {}", codes[3]);

    match codes.get(2) {
        Some(code) => println!("Lookup [Index 2]: Found status code: {}", code),
        None => println!("Lookup [Index 2]: Not found!"),
    }

    match codes.get(9) {
        Some(code) => println!("Lookup [Index 9]: Found status code: {}", code),
        None => println!("Lookup [Index 9]: ⚠️ Status code at index 9 does not exist! (Safely handled, zero crashes)"),
    }

    println!("=================================");
}
```

---

## 🧩 Key Takeaways

1. **Direct Indexing (`codes[1]`):**
   * Constant-time element lookup; a runtime bounds check may be optimized away when provably unnecessary.
   * If you try `codes[9]`, the program would crash immediately with `panic: index out of bounds`.
2. **Crash-Free Lookups (`codes.get(index)`):**
   * Returns an `Option<&T>`:
     - `Some(&value)` when valid.
     - `None` when outside bounds.
   * By combining `.get()` with `match`, these lookups handle unexpected indices without an indexing panic. Other failures remain possible.

## Everyday picture and symbols

Think of checking whether a street address actually exists. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `[i]` | Index; panics if a runtime index is invalid |
| `.get(i)` | Return an optional borrowed element |
| `Some(code) =>` | Name and handle a found element |
| `None =>` | Handle absence |

## Common pitfall

An empty collection has no last index. `.get()` handles invalid indices without indexing; it does not promise the whole application cannot crash.

## Expected output

```text
=== SERVER STATUS CODE LOOKUP ===
Direct check [Index 1]: 204
Direct check [Index 3]: 404
Lookup [Index 2]: Found status code: 301
Lookup [Index 9]: ⚠️ Status code at index 9 does not exist! (Safely handled, zero crashes)
=================================
```
