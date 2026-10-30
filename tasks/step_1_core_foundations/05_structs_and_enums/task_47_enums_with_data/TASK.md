# Task 47: User Session Activity Logger (Enums Holding Data)

## 🎯 What is Your Task?
Write a Rust program to log various user actions using an enum with diverse payloads:
1. Define an enum `UserActivity` with `#[derive(Debug)]`:
   - `Login(String)`: holding username
   - `Click { x: i32, y: i32 }`: holding coordinates
   - `Purchase { item: String, amount: f64 }`: holding item name and price
   - `Logout`: holding no data
2. In `main()`:
   - Create `a1`: Login for `"Zubair99"`.
   - Create `a2`: Click at `x: 340, y: 720`.
   - Create `a3`: Purchase of `"Wireless Headphones"` for `$89.99`.
   - Create `a4`: Logout.
   - Print all four activities with `{:?}`.

---

## 📋 Expected Output on Your Screen
```text
=== USER SESSION EVENT LOG ===
Event 1: Login("Zubair99")
Event 2: Click { x: 340, y: 720 }
Event 3: Purchase { item: "Wireless Headphones", amount: 89.99 }
Event 4: Logout
==============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Define the enum variants holding different types of data.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_47_enums_with_data
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Choose tuple-like, named-field, or empty syntax to match each variant’s definition.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Create a1 through a4 with `UserActivity::...`, using String::from for owned text, then print each with `{:?}`.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

A variant is a member of UserActivity, not a separate struct type. Its payload is accessed through a matching pattern when needed.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.
