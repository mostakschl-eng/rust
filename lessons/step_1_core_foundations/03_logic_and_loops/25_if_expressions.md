# Lesson 25: `if` as an Expression (Assigning Values from Decisions)

Welcome to Lesson 25! 

In other programming languages (like Python, Java, or C), an `if` block is just a statement that executes lines of code.
In Rust, an `if` block has a superpower: **It can produce a value directly and assign it to a variable!**

---

## 1. The Real-Life Analogy: The Automatic Vending Machine

Imagine you stand in front of a soda vending machine:
* You press **Button A** (Cold Water) or **Button B** (Orange Juice).
* The machine makes a decision and **drops the drink directly into your hands**!
* You take that drink and place it into your backpack (`let drink = ...`).

In Rust, instead of declaring a mutable variable and assigning it inside curly braces:
```rust
// ❌ The old, clumsy way:
let status;
if age >= 18 {
    status = "Adult";
} else {
    status = "Minor";
}
```

You can assign it directly in **one clean, beautiful sentence**:
```rust
// ✅ The elegant Rust way:
let status = if age >= 18 { "Adult" } else { "Minor" };
```

---

## 2. Working Code Example: `let result = if ...`

```rust
fn main() {
    let score = 85;

    // Direct assignment from if/else expression:
    let feedback = if score >= 80 {
        "Excellent job! 🌟"
    } else {
        "Keep practicing! 💪"
    }; // Notice the semicolon here at the end of the `let` statement!

    println!("Exam Score: {}", score);
    println!("Feedback:   {}", feedback);
}
```

### The Output on Screen:
```text
Exam Score: 85
Feedback:   Excellent job! 🌟
```

Notice:
1. No semicolons inside `{ "Excellent job! 🌟" }` because we want to **return** the text!
2. A semicolon `;` is placed at the very end of `};` because it completes the `let feedback = ...;` assignment.

---

## 3. The Golden Rule: Both Branches MUST Return the Same Type!

Because Rust is strictly typed, the variable box being created has a fixed type.
Therefore:
> **The `if` branch and the `else` branch MUST return the exact same data type!**

```rust
let number = if is_sunny {
    100       // Returns integer (i32)
} else {
    "Rainy"   // ❌ ERROR! Cannot return text (&str) for an integer!
};
```
* **What Rust says:** `error[E0308]: `if` and `else` have incompatible types`.  
  Rust protects you: *"You cannot have a box that is a number on Tuesdays and a string on Wednesdays!"*

---

## 4. Summary Checklist
- [x] In Rust, `if` is an expression that returns a value.
- [x] Assign directly: `let result = if condition { A } else { B };`.
- [x] Semicolon goes at the end of `};`.
- [x] Both `if` and `else` branches must return the exact same data type.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/03_logic_and_loops/task_25_if_expressions/TASK.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_25_if_expressions/TASK.md)**.
1. Read the task requirements.
2. Build an e-commerce shipping cost calculator assigning shipping fees directly via an `if` expression.
3. Run `cargo run` and test your expression assignments!
