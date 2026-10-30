# Lesson 03: Multi-Line Printing & Leaving Notes with Comments

Welcome to Lesson 03! 

In this lesson, we will discover two essential coding superpowers:
1. **Comments:** Leaving secret sticky notes inside your code that computers ignore, but humans love.
2. **Special Characters (`\n`, `\t`):** Formatting text into neat tables, banners, and multi-line paragraphs.

---

## 1. What Are Comments & Why Do We Need Them?

Imagine you are fixing an electric circuit or packing boxes in your attic. 
You write on a piece of masking tape:
> *"Important: Red wire connects to generator"* or *"Box 3: Winter clothes"*.

The boxes don't care about the tape. The tape is for **you and other humans** so nobody makes a mistake later.

In programming:
* You write code for the **computer** to execute.
* You write **comments** for **humans** (yourself tomorrow, or your teammate) to understand *why* you wrote the code.

> **Rule:** Ordinary comments do not execute. Documentation comments such as `///` can also become generated documentation.

---

## 2. Two Ways to Write Comments in Rust

### Style 1: Single-Line Comment (`//`)
Outside a string literal, `//` begins a comment that continues to the end of that line:

```rust
fn main() {
    // This is a note: Greeting the user
    println!("Hello, Rahim!"); // You can also put comments at the end of a line!
}
```

### Style 2: Multi-Line Block Comment (`/* ... */`)
If you have a long story or paragraph to write, start with `/*` and finish with `*/`:

```rust
/*
  This is a big comment block.
  I can write 10 lines here.
  Rust ignores all of it!
*/
fn main() {
    println!("Comments are awesome!");
}
```

---

## 3. Formatting Tricks: Newlines (`\n`) and Tabs (`\t`)

What if you want to print 3 lines without writing 3 separate `println!` statements?
You can use **Escape Sequences** inside your double quotes:

| Symbol | What is it called? | What does it do? |
|---|---|---|
| `\n` | Newline character | Hits the "Enter" key on your keyboard and starts a new line. |
| `\t` | Tab character | Advances to a terminal tab stop; spacing depends on the terminal. |
| `\"` | Escaped quote | Prints an actual quotation mark `"` on screen without confusing Rust. |

### Code Example:
```rust
fn main() {
    println!("Line 1\nLine 2\nLine 3");
    println!("Item:\tPrice:\nApple\t$1.00\nBanana\t$0.50");
    println!("He said: \"Rust is amazing!\"");
}
```

### The Output on Screen:
```text
Line 1
Line 2
Line 3
Item:	Price:
Apple	$1.00
Banana	$0.50
He said: "Rust is amazing!"
```

---

## 4. Summary Checklist
- [x] `//` writes a single-line comment.
- [x] `/* ... */` writes a multi-line comment block.
- [x] `\n` jumps down to a brand new line.
- [x] `\t` inserts a tab; its visible width depends on terminal tab stops.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/01_baby_steps/task_03_comments_and_banner/TASK.md`](../../../tasks/step_1_core_foundations/01_baby_steps/task_03_comments_and_banner/TASK.md)**.
1. Read the task requirements.
2. Build a decorative terminal banner with clean comments explaining your code.
3. Run `cargo run` and admire your formatted banner!

## Common pitfalls and limits

A comment marker inside a quoted string is text, not a comment. Tabs do not guarantee equally wide columns in every terminal; use formatting widths when alignment matters.
