# Task 68: Mission Dispatch File Reader (std::fs::read_to_string)

## 🎯 What is Your Task?
Write a Rust program to read and analyze mission logs from disk:
1. Define a helper function `count_file_stats(text: &str) -> (usize, usize)`:
   - Returns a tuple `(text.lines().count(), text.split_whitespace().count())`.
2. In `main()`:
   - Write a test log `"sample_mission.txt"` with:
     ```text
     Mission: Alpha
     Target: Mars Base 1
     Status: Active
     ```
   - Read the file using `std::fs::read_to_string("sample_mission.txt")`.
   - Print the file contents, total lines, and total words.
   - Attempt to read a missing file `"non_existent.txt"` and handle `Err` gracefully with `match`!

---

## 📋 Expected Output on Your Screen
```text
=== MISSION LOG FILE READER ===
[1. Reading Existing Mission Log]
File Contents:
Mission: Alpha
Target: Mars Base 1
Status: Active

Line Count: 3
Word Count: 7

[2. Reading Non-Existent File]
⚠️ Handled Gracefully: The system cannot find the file specified. (os error 2)
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_68_reading_files/src/main.rs).
2. Use `std::fs::read_to_string`.
3. Handle results safely using `match`.
4. Test:
   ```bash
   cd tasks/07_errors_and_io/task_68_reading_files
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_68_reading_files/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_68_reading_files/solution/explain.md)

