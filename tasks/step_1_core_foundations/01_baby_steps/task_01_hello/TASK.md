# Task 01: Hello World & Personal Introduction

## 🎯 What is Your Task?
Write a Rust program that prints **two lines** to the screen:
1. Line 1: Greet the world with your name (for example: `Hello, I am Rahim!`).
2. Line 2: Tell the computer your learning goal (for example: `I am starting my Rust developer journey!`).

---

## 📋 Expected Output on Your Screen
When you run your program, your terminal should look like this:

```text
Hello, I am Rahim!
I am starting my Rust developer journey!
```
*(You can use your real name or nickname!)*

---

## 🛠️ How to Do This (Step-by-Step):

1. Open the file: [`src/main.rs`](src/main.rs).
2. You will see `// TODO: Write your two println! statements here`.
3. Write your two `println!("...");` lines inside the `{` and `}` curly braces.
4. Open your terminal in this folder and run:
   ```bash
   cargo run
   ```
5. Look at the terminal: did it print both lines? If yes, congratulations! You solved Task 01!

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint 1: What command prints text?</b></summary>

Remember Lesson 01: `println!("Your text here");` prints text on the screen. 
Make sure your text is wrapped in double quotes `"..."`.
</details>

<br>

<details>
<summary><b>🔍 Hint 2: Semicolon reminder</b></summary>

Use semicolons after these printing statements. A final `println!` can omit one, but keeping it here is consistent:
```rust
fn main() {
    println!("Line 1");
    println!("Line 2");
}
```
</details>

---

## 🔒 Solution (Only if you need to compare!)
Once you finish your code (or if you are totally stuck), check:
* Official Working Code: [`solution/solution.rs`](solution/solution.rs)
* Why it works & pitfalls: [`solution/explain.md`](solution/explain.md)

