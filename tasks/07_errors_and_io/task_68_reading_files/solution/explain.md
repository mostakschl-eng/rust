# Solution Explanation: Task 68 (Reading Files)

Let's dissect how file loading interacts with Rust's `Result` type to prevent unhandled filesystem errors!

---

## 🔍 The Solution Code

```rust
use std::fs;

fn count_file_stats(text: &str) -> (usize, usize) {
    (text.lines().count(), text.split_whitespace().count())
}

fn main() {
    let sample_file = "sample_mission.txt";
    match fs::read_to_string(sample_file) {
        Ok(contents) => {
            println!("File Contents:\n{}", contents.trim());
            let (lines, words) = count_file_stats(&contents);
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## 🧩 Key Takeaways

1. **`std::fs::read_to_string`:**
   * Allocates a heap `String` and populates it with the file contents in one call.
   * Automatically closes the OS file descriptor when complete.
2. **Crash-Free Fallback:**
   * If a file doesn't exist, Rust does not crash with an uncaught exception; it returns `Err(std::io::Error)`, which your `match` arm catches smoothly.

