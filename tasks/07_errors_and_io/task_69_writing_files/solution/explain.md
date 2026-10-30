# Solution Explanation: Task 69 (Writing Files)

Let's dissect how file writing and appending work seamlessly with Rust I/O!

---

## 🔍 The Solution Code

```rust
use std::fs::{self, OpenOptions};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let log_file = "server_log.txt";

    // 1. Write or Overwrite
    fs::write(log_file, "[CONFIG] Port=8080 | Mode=Production")?;

    // 2. Open in Append Mode
    let mut file = OpenOptions::new()
        .append(true)
        .open(log_file)?;

    writeln!(file, "\n[EVENT] Security Handshake Established")?;

    // 3. Read back
    let saved_data = fs::read_to_string(log_file)?;
    println!("{}", saved_data.trim());

    Ok(())
}
```

---

## 🧩 Key Takeaways

1. **`std::fs::write`:**
   * One-line convenience function for saving text or byte buffers.
   * Completely overwrites any existing file at that path.
2. **`OpenOptions`:**
   * Configures file open behavior (`.read()`, `.write()`, `.append()`, `.create()`).
   * Essential for writing persistent transaction or activity logs without wiping prior records.
3. **`writeln!` macro:**
   * Like `println!`, but writes directly into an open file buffer with an automatic trailing newline.

