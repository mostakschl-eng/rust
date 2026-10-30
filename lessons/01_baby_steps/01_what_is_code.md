# Lesson 01: What is Programming & Your First Rust Program

Welcome! If you have **never written a single line of code in your life**, this lesson is made especially for you.

We will explain everything in simple English, with real-life examples, small code snippets, and every single symbol broken down. Nothing is skipped.

---

## 1. First, What is Programming?

Imagine you bought a brand new computer or a smartphone. 
It has powerful hardware inside, but by itself, it is just a box of electronics. It cannot read your mind.

If you want the computer to do something:
* Calculate your monthly expenses.
* Show a picture on the screen.
* Play a video game.
* Send a message over the internet.

You have to tell it **step-by-step** what to do.

> **Programming** simply means writing down instructions that tell a computer what to do.  
> The instructions you write are called **code**.  
> The person who writes these instructions is called a **programmer** or **developer**.

---

## 2. What is Rust?

Computers only understand electric signals: ones and zeros (`1`s and `0`s). 
Humans cannot write millions of ones and zeros easily. So humans invented **programming languages** that look more like English.

**Rust** is a modern, super-fast programming language created to build reliable and safe software.

### Why do people love Rust?
1. **It is Lightning Fast:** Programs written in Rust run as fast as the physical computer hardware can go.
2. **It Prevents Crashes:** In many other languages, a tiny mistake can crash an entire company's server. Rust checks your code thoroughly before running it and catches mistakes early.
3. **It has a Friendly Compiler:** When you make a mistake, Rust doesn't just say *"Error"*. It acts like a patient teacher and often tells you: *"Hey, you forgot a semicolon on line 3, here is how to fix it!"*

---

## 3. Your First Rust Code

Here is a complete, working Rust program:

```rust
fn main() {
    println!("Hello, World!");
}
```

When you run this program on your computer, here is the exact output on your terminal screen:

```text
Hello, World!
```

---

## 4. Understanding the Code (Piece by Piece)

Let's not skip anything. Every single word and symbol on those three lines has a specific job:

| Code Symbol | What is it called? | What does it mean in easy English? |
|---|---|---|
| `fn` | Function keyword | Tells Rust: *"I am creating a reusable set of instructions called a function."* |
| `main` | Main name | The special starting name. This is the **front door** of your program. Rust always starts running here. |
| `()` | Parentheses | Holds extra inputs if needed. Right now, it's empty because `main` doesn't need any inputs. |
| `{` | Opening curly brace | The start of the instruction room. Everything inside `{` and `}` belongs to `main`. |
| `println!` | Print line macro | The **loudspeaker**. It prints whatever text you give it onto the screen, then moves down to a new line. |
| `(` and `)` | Parentheses | Wraps the message you want to print. |
| `"Hello, World!"` | Text in quotes | The actual words you want to show. Double quotes `"` tell Rust: *"This is text to read, not instructions to run."* |
| `;` | Semicolon | The **period / full stop (.)** at the end of an instruction sentence. It tells Rust: *"This instruction is finished."* |
| `}` | Closing curly brace | The end of the instruction room. It tells Rust: *"The main program is finished."* |

---

## 5. Printing Multiple Lines

What if you want to print more than one line of text?
You simply write multiple instructions, one after another, from top to bottom. Each instruction must end with a semicolon `;`.

```rust
fn main() {
    println!("Hello, my name is Rahim!");
    println!("I am learning Rust from zero.");
    println!("This is my third line!");
}
```

### The Output on Screen:
```text
Hello, my name is Rahim!
I am learning Rust from zero.
This is my third line!
```

Notice:
* The computer runs line 1 first: it prints the greeting.
* Then it runs line 2: it prints the second sentence.
* Then it runs line 3: it prints the third sentence.
* Every single line has its own `;` at the end!

---

## 6. Writing Notes in Code: Comments (`//`)

Sometimes, you want to write a note to yourself inside the code so you remember what it does tomorrow.
In Rust, anything you type after two slashes `//` is a **comment**. 

Rust completely ignores comments when running your program:

```rust
fn main() {
    // This is a note for myself: The line below greets the user
    println!("Welcome to Rust!"); 

    // Rust will ignore this line too!
}
```

Output:
```text
Welcome to Rust!
```

---

## 7. Common Beginner Mistakes (What Happens If You Make an Error?)

Let's look at mistakes every single beginner makes, so you don't feel scared if they happen to you:

### Mistake 1: Forgetting the Semicolon `;`
```rust
fn main() {
    println!("Hello, World!") // <-- Forgot semicolon!
}
```
**What Rust Tells You:**
```text
error: expected `;`, found `}`
  --> src/main.rs:2:30
   |
 2 |     println!("Hello, World!")
   |                              ^ help: add `;` here
```
Rust literally puts a small arrow `^` and says: `help: add ';' here`. That is why we say Rust is like a helpful teacher!

### Mistake 2: Using Single Quotes instead of Double Quotes
```rust
fn main() {
    println!('Hello, World!'); // <-- Single quotes!
}
```
**Why this fails:** In Rust, single quotes `'A'` are only for a **single character**. For full sentences, you must always use double quotes `"..."`.

---

## 8. Summary Checklist of Lesson 01
Before moving to your practice task, check if you understand these 4 things:
- [x] Code is instructions given to a computer.
- [x] Every Rust program must have a starting door called `fn main() { ... }`.
- [x] `println!("Your text here");` prints text to the screen.
- [x] Every instruction statement must end with a semicolon `;`.

---

## 🛠️ Ready for Your Hands-On Practice!

Now it is your turn to write this with your own hands!

👉 Open **[`tasks/01_baby_steps/task_01_hello/README.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_01_hello/README.md)**.
1. Open [`tasks/01_baby_steps/task_01_hello/src/main.rs`](file:///d:/rust-learning/tasks/01_baby_steps/task_01_hello/src/main.rs).
2. Type your greeting and your goal.
3. Run `cargo run` in your terminal!
