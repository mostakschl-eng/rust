# Task 07: Area & Perimeter Calculator (Function Return Values)

## 🎯 What is Your Task?
Write a Rust program with **two calculator functions**:

1. `calculate_area(width: i32, height: i32) -> i32`
   - Multiplies `width * height` and returns the area.
2. `calculate_perimeter(width: i32, height: i32) -> i32`
   - Calculates `2 * (width + height)` and returns the perimeter.

In `main()`:
- Calculate the area and perimeter of a room with `width = 10` and `height = 5`.
- Print both answers using `println!`!

---

## 📋 Expected Output on Your Screen
```text
Room Dimensions: 10 meters wide by 5 meters high
Total Area:      50 square meters
Total Perimeter: 30 meters
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Define the two functions using `-> i32` and leave off the semicolon on the math expression.
3. Call the functions in `main()` and store the results in variables.
4. Test with:
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_07_function_returns
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Treat area and perimeter as separate recipes: multiplication for area, twice the sum for perimeter.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Each recipe promises an i32 answer. Leave its final arithmetic expression without a semicolon, then store or print each returned value.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

