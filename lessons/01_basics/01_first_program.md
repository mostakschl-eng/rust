# Lesson 01: Your Very First Rust Program

Welcome to your very first step in coding! 

If you have never written a single line of code in your life, you are in the right place. We will take it slow, simple, and step-by-step.

---

## 1. What is Programming?

Imagine you have a super-smart robot assistant. 
It can calculate billions of numbers in a second, but it cannot guess what you want. 

If you want the robot to do something, you must give it clear, exact steps:
1. *"Wake up."*
2. *"Show this message on the screen."*
3. *"Done."*

These written instructions are called **code**. Writing code is called **programming**.

---

## 2. The Simplest Rust Program in the World

Here is a complete, working Rust program:

```rust
fn main() {
    println!("Hello, World!");
}
```

When a computer runs this code, it prints this exact message on your terminal screen:
```text
Hello, World!
```

---

## 3. Breaking Down the Code (Piece by Piece)

Let’s not treat this like magic. Every single character has a clear job:

| Code Part | Simple Meaning | Real-Life Analogy |
|---|---|---|
| `fn` | Short for **function** (a reusable block of instructions). | A recipe card in a cooking book. |
| `main` | The official name of the starting point. | The **front door** of your house. Every Rust program starts here. |
| `()` | Parentheses. Holds inputs if needed. | An empty basket waiting for ingredients (right now, it's empty). |
| `{` and `}` | Curly braces. Everything between them is the instructions. | The walls of the room where the work happens. |
| `println!` | Prints a line of text on your screen. *(Pronounced "print-line")*. | A loudspeaker announcing a message. |
| `"Hello, World!"` | The text you want to display (wrapped in double quotes). | The actual words being spoken. |
| `;` | A semicolon. Marks the end of an instruction. | The **period / full stop (.)** at the end of an English sentence. |

---

## 4. Why the Exclamation Mark `!` in `println!`?

You might wonder: *"Why is there an exclamation mark `!` after `println`?"*

In Rust:
* A normal function is written without `!` (like `say_hello()`).
* When you see a `!` at the end (like `println!`), it means it is a **Macro**. 
* Don't worry about the technical details now. Just think of a macro as a **special built-in superpower tool** provided by Rust to make printing text super easy.

---

## 5. What if You Want to Print Two Lines?

You can write instructions one after another. The computer runs them from top to bottom:

```rust
fn main() {
    println!("Hello, Rahim!");
    println!("I am learning Rust!");
}
```

Output:
```text
Hello, Rahim!
I am learning Rust!
```

Notice: Each line ends with a semicolon `;`.

---

## 6. What Did We Learn?
1. Every Rust program starts at `fn main() { ... }`.
2. Instructions go inside the curly brackets `{ }`.
3. `println!("...")` displays text on the screen.
4. Every instruction ends with a semicolon `;`.

---

## 🎯 Next Step: Practice!
Don't just read code—type it yourself! 

👉 Go to **[Task 01: Hello World](file:///d:/rust-learning/tasks/01_basics/task_01_hello/README.md)** to run your very first code.

