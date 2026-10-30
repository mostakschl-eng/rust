# Task 01: Hello World & Personal Introduction

## 🎯 The Goal
Write a Rust program that prints **two lines** to the screen:
1. A greeting with your name (for example: `Hello, I am Rahim!`).
2. A sentence about your coding journey (for example: `I am starting my Rust developer journey!`).

---

## 📋 Expected Output
When you run your program, your terminal should look similar to this:
```text
Hello, I am Rahim!
I am starting my Rust developer journey!
```
*(You can use your real name or nickname!)*

---

## 🛠️ Step-by-Step Instructions

1. Open the starter code file: [`src/main.rs`](file:///d:/rust-learning/tasks/01_basics/task_01_hello/src/main.rs).
2. Look for the `// TODO` comment.
3. Write your instructions inside the `fn main() { ... }` curly braces.
4. Open your terminal in this task folder and run:
   ```bash
   cargo run
   ```
5. Check your screen! Does it display the two lines?

---

## 💡 Hints (Only open if you need help!)

<details>
<summary><b>🔍 Hint 1: What command prints a line of text?</b></summary>

Remember Lesson 01: Rust has a macro called `println!` that speaks text to the screen. 
Make sure your text is wrapped in double quotes `"..."`.
</details>

<br>

<details>
<summary><b>🔍 Hint 2: Code structure clue</b></summary>

Your code will have two separate print lines inside `main`, each ending with a semicolon `;`:
```rust
fn main() {
    println!("Line 1 text");
    println!("Line 2 text");
}
```
</details>

---

## 🔒 Need to Check the Solution?
If you get stuck or want to compare your finished code with the official answer:
* Official Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/01_basics/task_01_hello/solution/solution.rs)
* Line-by-Line Explanation: [`solution/explain.md`](file:///d:/rust-learning/tasks/01_basics/task_01_hello/solution/explain.md)
