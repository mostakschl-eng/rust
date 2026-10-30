# Task 33: Weather Station Weekly Array

## 🎯 What is Your Task?
Write a Rust program to manage a weather station's weekday log using fixed-size arrays:
1. Declare `weekdays: [&str; 5]` with: `"Monday"`, `"Tuesday"`, `"Wednesday"`, `"Thursday"`, `"Friday"`.
2. Declare `temperatures: [i32; 5]` with: `[28, 31, 30, 27, 29]`.
3. Print Day 1 (`weekdays[0]`) and its temperature (`temperatures[0]`).
4. Print Day 5 (`weekdays[4]`) and its temperature (`temperatures[4]`).
5. Print the total days monitored using `.len()`.
6. Declare a default sensor array `sensor_status: [i32; 5]` initialized to `1` using the repetition shortcut `[1; 5]`, and print it using `{:?}`.

---

## 📋 Expected Output on Your Screen
```text
=== WEATHER STATION 5-DAY LOG ===
Monday: 28°C
Friday: 29°C
Total days logged: 5
Sensor Check: [1, 1, 1, 1, 1]
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/04_collections/task_33_arrays/src/main.rs).
2. Declare the two arrays and the repeated array.
3. Access index `0` and index `4`.
4. Test:
   ```bash
   cd tasks/04_collections/task_33_arrays
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/04_collections/task_33_arrays/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/04_collections/task_33_arrays/solution/explain.md)

