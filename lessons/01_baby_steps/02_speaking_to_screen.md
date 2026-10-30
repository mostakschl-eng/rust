# Lesson 02: Speaking to the Screen — Placeholders & Semicolons

Welcome to Lesson 02! 

In Lesson 01, you learned how to print simple fixed text like `"Hello, World!"`. 
In this lesson, we will learn how to make our messages **dynamic and smart** using Rust's special fill-in-the-blank superpower: **Placeholders `{}`**.

---

## 1. The Real-Life Analogy: Fill-in-the-Blank Forms

Imagine you are filling out a government form or an invitation card:

> *"Dear **[Name]**, welcome to **[City]**!"*

You don't print a brand new form for every person. The sentence stays the same, and you simply fill in the blanks:
* For Rahim in Dhaka: *"Dear **Rahim**, welcome to **Dhaka**!"*
* For Sarah in London: *"Dear **Sarah**, welcome to **London**!"*

In Rust, the curly braces `{}` act as those exact **fill-in-the-blank slots**!

---

## 2. Using `{}` in `println!`

Look at this simple Rust code:

```rust
fn main() {
    println!("I have {} apples.", 5);
}
```

### What is the output on your screen?
```text
I have 5 apples.
```

### How did Rust do that?
1. Rust reads the text `"I have {} apples."`.
2. Whenever Rust spots `{}`, it knows: *"Aha! This is an empty slot."*
3. Rust looks after the comma `,` and grabs the number `5`.
4. Rust puts `5` right into the `{}` slot and prints: `I have 5 apples.`

---

## 3. Multiple Fill-in-the-Blanks

You can have as many `{}` slots as you want in a single sentence! Rust fills them in order from left to right:

```rust
fn main() {
    println!("My name is {} and I am {} years old.", "Rahim", 25);
    println!("Today is {} and the temperature is {} degrees.", "Friday", 30);
}
```

### The Output on Screen:
```text
My name is Rahim and I am 25 years old.
Today is Friday and the temperature is 30 degrees.
```

---

## 4. Understanding Every Piece of the Code

| Code Part | What does it do? |
|---|---|
| `println!` | The speaker macro that prints text on the screen. |
| `"My name is {} ..."` | The template sentence. Double quotes define text. |
| `{}` | An empty placeholder slot waiting to receive a value. |
| `,` (Comma) | Separates the template text from the values that will fill the slots. |
| `"Rahim"` | The value that goes into the **first** `{}` slot. |
| `25` | The value that goes into the **second** `{}` slot. |
| `;` | The semicolon marking the end of the instruction sentence. |

---

## 5. The Deep "Why": Why Semicolons `;` Are Mandatory

In English, if you write:
> *I went to school I ate an apple I came home*

It is confusing because there are no periods (.) between sentences.

To a computer processor, code is just a stream of characters. 
The semicolon `;` tells Rust: 
> *"This instruction is completely finished. Take a breath, finish this, and now you can read the next instruction."*

### What happens if you forget `;`?
```rust
fn main() {
    println!("First line") // <-- No semicolon!
    println!("Second line");
}
```
Rust compiler stops you and says:
```text
error: expected `;`, found `println`
  --> src/main.rs:3:5
   |
 2 |     println!("First line")
   |                           ^ help: add `;` here
```

---

## 6. Common Beginner Traps

### ❌ Trap 1: Not Enough Values for the Slots
```rust
println!("I bought {} apples and {} bananas.", 3); // 2 slots, but only 1 value!
```
* **What happens:** Rust will refuse to run and tell you: *"2 positional arguments in format string, but there is 1 argument"*. 
* **Fix:** Make sure the number of `{}` slots matches the number of values after the comma!

---

## 7. Summary Checklist
- [x] `{}` is an empty placeholder slot in `println!`.
- [x] Rust fills `{}` from left to right using the values provided after commas.
- [x] Semicolons `;` are like full stops at the end of every command.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/01_baby_steps/task_02_screen_speaking/TASK.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_02_screen_speaking/TASK.md)**.
1. Read the task requirements.
2. Open `src/main.rs` and write your code using `{}` placeholders.
3. Run `cargo run` and see your dynamic message!
