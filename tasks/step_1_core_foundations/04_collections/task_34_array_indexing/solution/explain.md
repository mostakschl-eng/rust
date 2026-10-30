# Solution Explanation: Task 34 (Array Indexing & Bounds Checking)

Let's dissect how Rust provides zero-cost bounds checking to protect systems from crashes and memory corruption!

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
   * Instantaneous lookups.
   * If you try `codes[9]`, the program would crash immediately with `panic: index out of bounds`.
2. **Crash-Free Lookups (`codes.get(index)`):**
   * Returns an `Option<&T>`:
     - `Some(&value)` when valid.
     - `None` when outside bounds.
   * By combining `.get()` with `match`, your program will never crash from unexpected indices!

