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
Word Count: 8

[2. Reading Non-Existent File]
⚠️ Handled Gracefully: The system cannot find the file specified. (os error 2)
===============================
```

---

The missing-file diagnostic above is a Windows example; wording differs across systems. That branch assumes `non_existent.txt` is absent. The sample setup overwrites `sample_mission.txt` in the working directory. Stop and report an error if writing the sample fails.

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Use `std::fs::read_to_string`.
3. Handle results safely using `match`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/07_errors_and_io/task_68_reading_files
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Handle the sample write result before attempting to read it.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Count lines and words from the loaded text, then use a separate match for the deliberately missing path.

</details>

**Before running:** The mission text has 3 lines and 8 whitespace-separated words. A trailing newline does not add a lines() item. Error wording differs by OS; relative paths use the process working directory.
