# Task 04: Creating & Calling Your First Custom Function

## 🎯 What is Your Task?
Write a Rust program that defines a reusable function named **`cheer()`**.

1. The `cheer()` function must print two lines:
   - Line 1: `"Rust is fast!"`
   - Line 2: `"Rust is safe!"`
2. Inside `fn main()`, you must call `cheer()` **three times**!

---

## 📋 Expected Output on Your Screen
```text
Cheering for the first time:
Rust is fast!
Rust is safe!
Cheering for the second time:
Rust is fast!
Rust is safe!
Cheering for the third time:
Rust is fast!
Rust is safe!
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Create your `fn cheer() { ... }` function outside of `main`.
3. Inside `main()`, call `cheer();` three times.
4. Test it in the terminal:
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_04_first_function
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Defining a function stores the recipe; it does not run the recipe. Put the two cheer messages in that body.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Call the same named function three times from main, with an introductory message before each call.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

