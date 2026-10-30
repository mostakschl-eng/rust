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

1. Open [`src/main.rs`](src/main.rs).
2. Declare the two arrays and the repeated array.
3. Access index `0` and index `4`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/04_collections/task_33_arrays
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Each array has five elements, so both types include `; 5`.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Monday is index 0 and Friday is index 4. `[1; 5]` repeats a value; `[1, 5]` has only two items.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

Array length is part of its type; it does not mean arrays must live on the stack.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
