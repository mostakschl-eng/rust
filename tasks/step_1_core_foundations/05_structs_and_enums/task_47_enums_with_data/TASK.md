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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_47_enums_with_data/src/main.rs).
2. Define the enum variants holding different types of data.
3. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_47_enums_with_data
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_47_enums_with_data/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_47_enums_with_data/solution/explain.md)

