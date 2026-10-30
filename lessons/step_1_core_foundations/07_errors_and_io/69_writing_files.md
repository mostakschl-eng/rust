# Lesson 69: Writing Data to Files (`std::fs::write`)

In Lesson 68, we read existing files from the hard drive.  
Now, how do we **save data to a file**?
* Saving user game progress.
* Writing application logs.
* Exporting reports or invoices.

In Rust, the simple one-call way to write or overwrite a file is **`std::fs::write()`**! ✍️💾

---

## 1. The Real-Life Analogy: The Typewriter & File Folder 📑

Imagine taking a clean sheet of paper, typing an official report, and placing it inside a manila folder:
* If the file folder doesn't exist yet: The office clerk **creates a new folder** automatically!
* If a file with that name already exists: The clerk **replaces the old contents** with your fresh new document!
* What could fail?
  1. The hard drive is 100% full (no disk space!).
  2. The folder is write-protected (Permissions denied!).

Therefore:
> **`fs::write()` returns a `Result<(), std::io::Error>`!**  
> On success, it returns `Ok(())` (empty unit value).  
> On failure, it returns `Err(io_error)`.

---

## 2. Writing Text to a File in One Line

```rust
use std::fs;

fn main() {
    let file_path = "output_report.txt";
    let report_content = "=== SYSTEM AUDIT REPORT ===\nAll 12 microservices healthy.\nLatency: 14ms\n";

    // Write contents to disk:
    match fs::write(file_path, report_content) {
        Ok(()) => println!("Successfully saved data to '{}'!", file_path),
        Err(err) => println!("⚠️ Failed to write to file: {}", err),
    }
}
```

### 📋 Expected Terminal Output:
```text
Successfully saved data to 'output_report.txt'!
```

After success, the file contains these bytes. This is not a promise of durability after power loss; advanced durable writing needs additional synchronization and recovery design.

---

## 3. Appending Data with `OpenOptions`

`fs::write()` completely overwrites existing files.  
What if you are building an **activity logger** and want to **append new lines to the end of the file** without erasing past history?

Use **`std::fs::OpenOptions`**:

```rust
use std::fs::OpenOptions;
use std::io::Write; // Brings file-writing trait methods into scope

fn append_log_entry(file_path: &str, entry: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true) // Create file if it doesn't exist
        .append(true) // Append to end of file
        .open(file_path)?;

    writeln!(file, "{}", entry)?; // Appends line with automatic newline '\n'
    Ok(())
}

fn main() -> std::io::Result<()> {
    append_log_entry("app.log", "[INFO] User logged in: admin")?;
    append_log_entry("app.log", "[INFO] Database backup completed")?;

    println!("Appended entries to app.log successfully!");
    Ok(())
}
```

---

## 4. Summary: File Writing Methods

| Tool | Purpose | Behavior if file exists |
|---|---|---|
| `fs::write("file", data)` | Quick one-shot write | Overwrites entire file |
| `OpenOptions::new().append(true)` | Continuous logging | Appends to the bottom |
| `writeln!(file, "format", ...)` | Writes a formatted line | Writes at the current position, or at the end when opened for append |

---

## 5. Quick Recap Checklist
- [x] Write files with `std::fs::write(path, data)`.
- [x] Returns `std::io::Result<()>`.
- [x] Automatically creates the file if it doesn't exist.
- [x] Overwrites existing file contents.
- [x] Use `OpenOptions::new().append(true)` for append-only logging.

---


## Symbols and Practical Boundaries

Replacing a report discards the previous pages; appending a log adds a new page without erasing the old entries.

| Symbol or term | Meaning |
|---|---|
| `fs::write` | create or truncate file |
| `.append(true)` | append writes |
| `writeln!` | formatted write plus newline |
| `?` | propagate failure |

**Common pitfall:** fs::write overwrites existing contents and is not an atomic backup or durable transaction. It creates the file, not missing parent directories. Each run of this demo resets its named log.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/07_errors_and_io/task_69_writing_files/TASK.md`](../../../tasks/step_1_core_foundations/07_errors_and_io/task_69_writing_files/TASK.md).
* Next lesson: [Lesson 70: Organizing Code with Modules (`mod`, `pub`, `use`)](70_modules_and_packages.md).
