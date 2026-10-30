# Beginner Guide: How to Read Rust Compiler Errors (`compiler_errors.md`)

## The #1 Rule: Don't Panic!
In other programming languages, errors can be confusing red walls of text.
In Rust, **the compiler is your personal teacher**. It tells you:
1. Exactly which file and line number had the problem.
2. What it expected vs what it found.
3. Very often, a `help:` line telling you the exact fix!

---

## 1. "expected `;`, found ..."
### What it means:
You forgot to put a semicolon `;` at the end of an instruction.
```text
error: expected `;`, found `println`
  --> src/main.rs:3:21
```
### How to fix:
Look at the line above or the line mentioned, and add `;` at the end.

---

## 2. "cannot assign twice to immutable variable `x`"
### What it means:
By default, variables in Rust are locked (immutable). You tried to change its value without making it changeable.
```text
error[E0384]: cannot assign twice to immutable variable `count`
  --> src/main.rs:4:5
   |
 2 |     let count = 5;
   |         ----- first assignment to `count`
 3 |     count = 6;
   |     ^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable: `mut count`
```
### How to fix:
Change `let count = 5;` to `let mut count = 5;`.

---

## 3. "mismatched types"
### What it means:
You told Rust to expect one type of data (like a whole number `i32`), but you gave it text or a decimal number instead.
```text
error[E0308]: mismatched types
  --> src/main.rs:2:22
   |
 2 |     let age: i32 = "twenty";
   |              ---   ^^^^^^^^ expected `i32`, found `&str`
```
### How to fix:
Make sure the value matches the type: `let age: i32 = 20;`.

---

## 4. "cannot find value `x` in this scope"
### What it means:
You are trying to use a variable or function name that doesn't exist. Usually this is a spelling mistake (typo) or you haven't declared it with `let` yet.
```text
error[E0425]: cannot find value `totall` in this scope
  --> src/main.rs:3:14
help: a local variable with a similar name exists: `total`
```
### How to fix:
Check your spelling!

---

## 5. "warning: unused variable"
### What it means:
This is NOT an error. Your program will still run! Rust is just letting you know: *"Hey, you created a variable named `x`, but you never used it anywhere."*
### How to fix:
Either use the variable, or if you plan to use it later, put an underscore in front: `let _x = 10;`.

