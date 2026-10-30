# Task 03: Terminal Banner & Comments

## 🎯 What is Your Task?
Write a Rust program that prints a neat decorative **Welcome Banner** using comments (`//`), escape characters (`\n`), and clean formatting.

Your program must:
1. Include at least **two comments** (`//`) explaining what the code is doing.
2. Print a decorative box banner around your name and title.

---

## 📋 Expected Output on Your Screen
```text
========================================
       WELCOME TO MY RUST JOURNEY       
========================================
Student Name:  Rahim
Goal:          Master Rust from Zero!
Status:        Level 1 in Progress
========================================
```
*(You can customize the text inside the banner!)*

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_03_comments_and_banner/src/main.rs).
2. Add your comments starting with `//`.
3. Use `println!` statements to draw the border lines (`====`) and text.
4. Test your banner in the terminal:
   ```bash
   cd tasks/01_baby_steps/task_03_comments_and_banner
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Border design</b></summary>

You can print borders using repeating equal signs:
```rust
println!("========================================");
println!("       WELCOME TO MY RUST JOURNEY       ");
println!("========================================");
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_03_comments_and_banner/solution/solution.rs)
* Explanation: [`solution/explain.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_03_comments_and_banner/solution/explain.md)
