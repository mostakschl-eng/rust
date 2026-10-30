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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_04_first_function/src/main.rs).
2. Create your `fn cheer() { ... }` function outside of `main`.
3. Inside `main()`, call `cheer();` three times.
4. Test it in the terminal:
   ```bash
   cd tasks/01_baby_steps/task_04_first_function
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Function structure</b></summary>

You write the function definition above or below `main()`:
```rust
fn cheer() {
    println!("Rust is fast!");
    println!("Rust is safe!");
}

fn main() {
    cheer();
}
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_04_first_function/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_04_first_function/solution/explain.md)

