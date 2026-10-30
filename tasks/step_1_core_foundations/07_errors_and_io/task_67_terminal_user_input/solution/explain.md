# Solution Explanation: Task 67 (Terminal Input)

Let's dissect how raw user keystrokes are cleaned, parsed, and converted safely!

---

## 🔍 The Solution Code

```rust
use std::io;

fn parse_terminal_input(raw: &str) -> (String, u32) {
    let clean = raw.trim();
    let mut parts = clean.split_whitespace();
    let name = parts.next().unwrap_or("Guest").to_string();
    let age = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(18);
    (name, age)
}

fn main() -> io::Result<()> {
    assert_eq!(parse_terminal_input("  Tanvir 22\r\n"), ("Tanvir".to_string(), 22));
    assert_eq!(parse_terminal_input(""), ("Guest".to_string(), 18));
    assert_eq!(parse_terminal_input("Maya invalid"), ("Maya".to_string(), 18));
    assert_eq!(parse_terminal_input("Maya -1"), ("Maya".to_string(), 18));
    assert_eq!(parse_terminal_input("Maya 4294967296"), ("Maya".to_string(), 18));

    println!("=== TERMINAL INPUT SANITIZER ===");
    println!("Enter a name and age (for example: Tanvir 22):");
    let mut raw_input = String::new();
    io::stdin().read_line(&mut raw_input)?;
    let (name, age) = parse_terminal_input(&raw_input);
    println!("Sanitized Name:  {}", name);
    println!("Parsed Age:      {} years old", age);
    println!("================================");
    Ok(())
}
```

---

## 🧩 Key Takeaways

1. **Whitespace policy:**
   * Removes `\r\n` line endings produced by keyboards when users strike Enter.
2. **Safe Parsing with Fallbacks:**
   * `.parse::<u32>().ok()` transforms `Result<u32, ParseIntError>` into `Option<u32>`.
   * `.unwrap_or(18)` implements the requested demo fallback; a real age form should usually report invalid input instead.


## Reading the Result Carefully

A microphone records into a reusable tape. read_line appends to an existing buffer, so clear it before reusing it for a fresh line.

| Symbol or term | Meaning |
|---|---|
| `io::stdin()` | input handle |
| `.read_line(&mut buf)` | append one UTF-8 line |
| `.trim()` | borrowed whitespace-trimmed view |

read_line returns a byte count; zero means EOF. trim removes all surrounding whitespace, not just Enter. This exercise permits one-word names and demo defaults, not identity or age verification.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.

`read_line` really reads stdin and can wait for Enter. Its `Result` is propagated by `?`. It returns zero at EOF, leaving an empty buffer and therefore the requested defaults. The assertions run silently before reading, so they verify parser edge cases without adding output. The parser uses an iterator because no intermediate Vec is needed.
