# Lesson 20: Becoming a Compiler Doctor (Diagnosing & Fixing Errors)

Welcome to Lesson 20! 

This is the graduation lesson for **Level 2: Basic Types & Arithmetic**!
Today, we are going to give you a superpower that turns beginners into confident developers: **Understanding how to diagnose and cure any Rust compiler error!**

---

## 1. The Mindset: The Compiler is Your Friend, Not Your Enemy

In many old programming languages, when code fails, you get cryptic errors like `Segmentation Fault (core dumped)` with zero explanation.

In Rust, the creators spent thousands of hours making compiler error messages as friendly, human, and helpful as possible.

Think of the Rust compiler as a **strict, loving flight instructor**:
* It will not let you fly the plane with a loose bolt.
* It stops you on the ground *before* you crash!
* It points its finger directly at the loose bolt and hands you the wrench to fix it!

---

## 2. Anatomy of a Rust Error Message

Whenever an error happens, Rust prints 4 clear parts:

```text
error[E0308]: mismatched types
  --> src/main.rs:3:20
   |
 3 |     let age: u8 = "twenty";
   |              --   ^^^^^^^^ expected `u8`, found `&str`
   |              |
   |              expected due to this
help: a character literal may also be used here: `'twenty'`
```

Look at the 4 pieces:
1. **Error Code (`[E0308]`):** The official disease code. You can type `rustc --explain E0308` in your terminal to see a full textbook explanation with examples!
2. **File & Line (`src/main.rs:3:20`):** File `src/main.rs`, Line `3`, Character `20`. Click or open that exact line!
3. **The Finger Arrow (`^^^^^^^^`):** Points directly to the exact word causing the problem (`"twenty"`).
4. **The `help:` Line:** Tells you how to fix it!

---

## 3. The Big 4 Level 1 & 2 Errors Decoded

| Error Code | Official Title | What It Means in Plain English | How to Cure It |
|---|---|---|---|
| **E0308** | `mismatched types` | You promised an integer or boolean, but gave text (or vice versa). | Match the value with the declared type, or use `as f64`. |
| **E0384** | `cannot assign twice to immutable variable` | You tried to update a variable, but forgot to unlock it! | Add `mut` where you declared the variable: `let mut x = ...`. |
| **E0425** | `cannot find value in this scope` | Typo in a variable name, or the variable was born in another room `{ }` that already closed! | Check spelling, or move declaration outside the inner block. |
| **E0277** | `cannot be formatted with the default formatter` | You tried to print a compound type (like a tuple) with `{}`. | Switch from `{}` to Debug format `{:?}`. |

---

## 4. Milestone 2: You Have Mastered Level 2!

🎉 Look at everything you now know:
- [x] Whole numbers, bit sizes, and signed `i` vs unsigned `u`.
- [x] Decimals and float precision with `f64`.
- [x] Booleans `true`/`false` and the `!` NOT operator.
- [x] Single characters and emojis with `char` and single quotes `' '`.
- [x] Standard math (`+`, `-`, `*`, `/`) and remainder modulus `%`.
- [x] Type casting with the `as` keyword.
- [x] Scope blocks `{ }` and memory cleanup.
- [x] Formatted printing with `{:.2}` and table alignment.
- [x] Debug printing with `{:?}`, `{:#?}`, and `dbg!`.
- [x] Reading and fixing compiler errors with confidence!

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_20_compiler_doctor/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_20_compiler_doctor/TASK.md)**.
1. Read the task requirements.
2. In this task, you are given a program with **3 intentional beginner compiler errors**. Your mission as the Compiler Doctor is to fix all three and make it run!
3. Run `cargo run` and celebrate graduating Level 2!
