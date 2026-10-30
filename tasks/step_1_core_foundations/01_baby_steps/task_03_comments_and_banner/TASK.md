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

1. Open [`src/main.rs`](src/main.rs).
2. Add your comments starting with `//`.
3. Use `println!` statements to draw the border lines (`====`) and text.
4. Test your banner in the terminal:
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_03_comments_and_banner
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Split the banner into a heading, details, and closing border. Comments are notes, so they will not appear in output.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Use a newline escape inside one string to print more than one line, and check whether println! adds the final newline.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Explanation: [`solution/explain.md`](solution/explain.md)

