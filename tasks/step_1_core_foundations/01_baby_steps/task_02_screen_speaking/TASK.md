# Task 02: Dynamic Screen Speaking with Placeholders `{}`

## 🎯 What is Your Task?
Write a Rust program that uses **`println!` with `{}` placeholders** to introduce yourself dynamically.

Your program must print two lines:
1. Line 1: Fill in your name and your city using two `{}` placeholders.
   * Example: `"Hello, my name is Rahim and I live in Dhaka."`
2. Line 2: Fill in your favorite number and programming language using two `{}` placeholders.
   * Example: `"My lucky number is 7 and my favorite language is Rust!"`

---

## 📋 Expected Output on Your Screen
```text
Hello, my name is Rahim and I live in Dhaka.
My lucky number is 7 and my favorite language is Rust!
```
*(You can use your real name, your own city, and your own lucky number!)*

---

## 🛠️ How to Do This (Step-by-Step):

1. Open the code file: [`src/main.rs`](src/main.rs).
2. Inside `fn main() { ... }`, write your two `println!` statements using `{}` slots.
3. Open your terminal in this folder and test it:
   ```bash
   cd tasks/step_1_core_foundations/01_baby_steps/task_02_screen_speaking
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint 1: Syntax pattern</b></summary>

Remember to place commas `,` between the template text and the values:
```rust
println!("Hello, my name is {} and I live in {}.", "YourName", "YourCity");
```
</details>

<br>

<details>
<summary><b>🔍 Hint 2: Numbers in placeholders</b></summary>

Numbers do not need quotation marks! You can pass them directly:
```rust
println!("My lucky number is {} and my language is {}.", 7, "Rust");
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

