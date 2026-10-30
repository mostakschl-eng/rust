# Task 69: Telemetry File Writer & Append Logger (std::fs::write)

## 🎯 What is Your Task?
Write a Rust program to save records to disk and append log entries:
1. Use `std::fs::write` to create a configuration file `"server_log.txt"` with:
   ```text
   [CONFIG] Port=8080 | Mode=Production
   ```
2. Append a new event using `std::fs::OpenOptions`:
   - Open `"server_log.txt"` with `.append(true).open(...)`.
   - Write `"\n[EVENT] Security Handshake Established"`.
3. Read the file back with `std::fs::read_to_string` and print all saved lines!

---

## 📋 Expected Output on Your Screen
```text
=== TELEMETRY FILE WRITER & LOGGER ===
Writing initial base configuration...
Appending runtime security event...

Reading final saved log from disk:
[CONFIG] Port=8080 | Mode=Production
[EVENT] Security Handshake Established
File operations complete! 💾
======================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_69_writing_files/src/main.rs).
2. Use `fs::write` and `OpenOptions::new().append(true)`.
3. Test:
   ```bash
   cd tasks/07_errors_and_io/task_69_writing_files
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_69_writing_files/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_69_writing_files/solution/explain.md)

