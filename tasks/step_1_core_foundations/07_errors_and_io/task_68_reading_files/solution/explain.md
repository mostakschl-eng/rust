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
2. **Explicit I/O Failure:**
   * If a file doesn't exist, Rust does not crash with an uncaught exception; it returns `Err(std::io::Error)`, which your `match` arm catches smoothly.


## Reading the Result Carefully

A filing clerk can read a folder only after it was successfully created. Do not report a new report if saving the sample failed.

| Symbol or term | Meaning |
|---|---|
| `fs::read_to_string` | load UTF-8 file |
| `.lines()` | in-memory line iterator |
| `.split_whitespace()` | word-like tokens |

The mission text has 3 lines and 8 whitespace-separated words. A trailing newline does not add a lines() item. Error wording differs by OS; relative paths use the process working directory.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.

The complete solution checks fs::write before reading, so it cannot accidentally analyze an old file after a failed sample write. Its silent assertions check the known sample as (3, 8) and empty text as (0, 0). The displayed OS error is platform-specific and assumes the missing path really is absent.
