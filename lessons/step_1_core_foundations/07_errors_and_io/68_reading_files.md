# Lesson 68: Reading Files from Disk (`std::fs::read_to_string`)

When your computer shuts down, everything in RAM (variables, vectors, integers) disappears forever.  
To build software that survives reboots, we must read and write data to the **Hard Drive / SSD**! 💾

In Rust, reading files from disk is handled by the **`std::fs` (File System)** module.  
The simplest and most common function is **`fs::read_to_string()`**! 📄

---

## 1. The Real-Life Analogy: The Filing Cabinet 🗄️

Imagine an office filing cabinet:
* Inside drawer "Documents" is a manila folder named `"report.txt"`.
* You walk up, pull out the paper document, and read all the printed words into your mind.
* What could go wrong?
  1. The folder doesn't exist (Someone misplaced it!).
  2. The drawer is locked (Permissions denied!).
  3. The paper is water-damaged or unreadable.

Because any of these real-world problems can happen:
> **`fs::read_to_string()` returns a `Result<String, std::io::Error>`!**  
> If successful, it hands you `Ok(file_contents)`.  
> If failed, it hands you `Err(io_error)`!

---

## 2. Reading an Entire File in One Line

Look at how concise Rust file reading is:

```rust
use std::fs;

fn main() {
    let file_path = "notes.txt";

    // Attempt to read the entire file into a String:
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("=== FILE CONTENTS ===");
            println!("{}", contents);
            println!("=====================");
            println!("Successfully read {} bytes from disk!", contents.len());
        }
        Err(error) => {
            println!("⚠️ Failed to read '{}': {}", file_path, error);
        }
    }
}
```

---

## 3. Reading Line-by-Line with `.lines()`

Once you have read the file contents into a `String`, you can iterate through it line by line:

```rust
use std::fs;

fn main() {
    let content = fs::read_to_string("servers.txt").unwrap_or_else(|_| {
        String::from("127.0.0.1\n192.168.1.1\n10.0.0.5")
    });

    println!("=== SERVER ROSTER ===");
    for (line_number, line) in content.lines().enumerate() {
        println!("Server #{}: {}", line_number + 1, line);
    }
}
```

### 📋 Expected Terminal Output:
```text
=== SERVER ROSTER ===
Server #1: 127.0.0.1
Server #2: 192.168.1.1
Server #3: 10.0.0.5
```

---

## 4. Reading Raw Binary Files: `fs::read`

What if you are reading an image (`.png`), an audio file (`.mp3`), or an executable binary rather than plain text?  
Plain text requires UTF-8 formatting. For raw binary files, use **`fs::read()`**:
* `fs::read("image.png") -> Result<Vec<u8>, std::io::Error>`
* Returns a vector of raw bytes (`Vec<u8>`)!

---

## 5. Quick Recap Checklist
- [x] Read text files using `std::fs::read_to_string("path")`.
- [x] Returns `Result<String, std::io::Error>`.
- [x] Process text line by line using `.lines()`.
- [x] For raw binary data (images, archives), use `std::fs::read()` returning `Vec<u8>`.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/07_errors_and_io/task_68_reading_files/TASK.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_68_reading_files/TASK.md).
* Next lesson: [Lesson 69: Writing Data to Files (`std::fs::write`)](file:///d:/rust-learning/lessons/07_errors_and_io/69_writing_files.md).
