# Task 11: System Specs & Data Sizes (Integer Types)

## 🎯 What is Your Task?
Write a Rust program that stores computer and game stats using the most appropriate **specific integer types**:

1. `freezer_temp: i8` = `-18` (Freezer temperature in Celsius, negative allowed)
2. `battery_percentage: u8` = `95` (Phone battery: 0 to 100, cannot be negative)
3. `active_players: u32` = `1_250_000` (Online game player count, positive, over 1 million)
4. `world_population: i64` = `8_000_000_000` (8 billion people, requires 64-bit size)

Print each stat with clean labels using `println!`!

---

## 📋 Expected Output on Your Screen
```text
=== SYSTEM & WORLD STATS ===
Freezer Temperature: -18 C
Battery Level:       95%
Online Players:      1250000
World Population:    8000000000
============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Declare variables with explicit type annotations: `let var_name: Type = value;`.
3. Print each variable.
4. Test with:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_11_integer_sizes
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Choose signed storage for a possible minus sign. Check that the maximum value fits the chosen number of bits.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Use a colon after each variable name to state the required type. Numeric underscores are only for source readability.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

