# Task 59: Waveform Telemetry Analyzer (Slices &[T] & &str)

## 🎯 What is Your Task?
Write a Rust program to extract and process data slices from strings and arrays without allocating new memory:
1. Define a helper function:
   ```rust
   fn sum_slice(data: &[i32]) -> i32
   ```
   - Calculates and returns the sum of all elements in the slice using a `for` loop.
2. In `main()`:
   - Given `let sentence = String::from("Rustacean Power");`:
     - Slice the first word: `&sentence[0..9]`
     - Slice the second word: `&sentence[10..]`
     - Print both words.
   - Given telemetry readings `let readings = [12, 45, 67, 89, 90, 34, 21];`:
     - Extract the peak readings at indices `2..5`: `let peak = &readings[2..5];`
     - Print the peak slice with `{:?}`.
     - Call `sum_slice(peak)` and print the total peak energy sum!

---

## 📋 Expected Output on Your Screen
```text
=== TELEMETRY SLICE ANALYZER ===
[1. Text Slices]
Word 1: 'Rustacean'
Word 2: 'Power'

[2. Numeric Array Slices]
Full Readings: [12, 45, 67, 89, 90, 34, 21] (Count: 7)
Peak Window:   [67, 89, 90] (Count: 3)
Peak Energy:   246 units
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_59_slices_deep_dive/src/main.rs).
2. Write `sum_slice(data: &[i32]) -> i32`.
3. Use range slicing `&data[start..end]`.
4. Test:
   ```bash
   cd tasks/06_ownership_and_memory/task_59_slices_deep_dive
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_59_slices_deep_dive/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_59_slices_deep_dive/solution/explain.md)

