# Lesson 09: Mutability (`mut` — Locked vs Changeable Boxes)

Welcome to Lesson 09! 

In this lesson, we will uncover one of Rust's **most famous safety principles**:  
**Why variables in Rust cannot be changed by default, and how to unlock them with `mut`!**

---

## 1. The Real-Life Analogy: The Locked Museum Case vs The Desk Drawer

Imagine two kinds of storage boxes in your house:

1. **A Sealed Glass Museum Case:**
   You put a museum artifact inside and lock it. Nobody can accidentally knock it over, swap it, or tamper with it. Once placed, it stays fixed forever.  
   In coding, this is called **Immutable** (*un-changeable*).

2. **An Open Desk Drawer:**
   You put a pencil inside. Later, you take the pencil out and put an eraser inside. The contents can change at any time.  
   In coding, this is called **Mutable** (*changeable*).

---

## 2. The Big Rust Surprise: Variables Are Locked by Default!

A binding controls how a name may be used in its scope. In Rust, `let` prevents reassignment unless you request mutability with `mut`. Other languages have their own binding rules too. 

In giant applications with millions of lines of code, someone accidentally changing a number in one corner of the program can crash the whole system!

Rust decided:
> **In Rust, every variable is locked (immutable) by default!**

Look at what happens if you try to change a normal variable:

```rust
fn main() {
    let score = 10;
    score = 20; // ❌ ERROR!
    println!("Score: {}", score);
}
```

### What Rust Tells You:
```text
error[E0384]: cannot assign twice to immutable variable `score`
  --> src/main.rs:3:5
   |
 2 |     let score = 10;
   |         ----- first assignment to `score`
 3 |     score = 20;
   |     ^^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable: `mut score`
```

Look at that `help:` line! Rust explains:
> *"Hey, you locked this box with `let score = 10;`. If you want to change it, tell me by writing `mut score`!"*

---

## 3. How to Unlock a Variable: The `mut` Keyword

If you are building a game score, a bank account, or a speedometer that **needs to change**, you add `mut` (short for **mutable**):

```rust
fn main() {
    let mut score = 10; // 🔓 We unlocked the box with 'mut'!
    println!("Starting score: {}", score);

    score = 20; // We changed the value!
    println!("Score after quest: {}", score);

    score = score + 50; // We added 50 bonus points!
    println!("Score with bonus: {}", score);
}
```

### The Output on Screen:
```text
Starting score: 10
Score after quest: 20
Score with bonus: 70
```

Notice: We only write `let mut` the **first time** we create the box. When changing the value later, we just write `score = 20;`.

---

## 4. Understanding Every Piece of the Code

| Code Part | What does it mean? |
|---|---|
| `let` | Create a new variable box in memory. |
| `mut` | Short for **mutable**. Unlocks the box so you can change what is inside later. |
| `score` | The name of the variable. |
| `score = 20;` | Replaces the old value inside `score` with `20`. |
| `score = score + 50;` | Reads the current score (20), adds 50 (70), and puts 70 back into the box! |

---

## 5. A Very Important Rule: You Cannot Change the Type!

Even if a box is unlocked with `mut`, **you cannot change the kind of data it holds**:

```rust
fn main() {
    let mut age = 25;
    age = "twenty-six"; // ❌ ERROR!
}
```
* **Why:** The box was created for whole numbers (`i32`). You cannot suddenly shove text (`&str`) into a number box!
* **Rust Error:** `error: mismatched types: expected integer, found '&str'`.

---

## 6. Summary Checklist
- [x] Variables created with `let x = 5;` are **locked (immutable)** by default.
- [x] To make a variable changeable, use: `let mut x = 5;`.
- [x] Updating a variable is written as: `x = 10;`.
- [x] You cannot change the data type of a variable even if it is `mut`.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/01_baby_steps/task_09_mutability_score/TASK.md`](../../../tasks/step_1_core_foundations/01_baby_steps/task_09_mutability_score/TASK.md)**.
1. Read the task requirements.
2. Build an arcade score tracker that starts at 0, gains points across 3 rounds, and displays the final score.
3. Run `cargo run` and watch the score climb!

## Common pitfalls and limits

mut permits reassignment; it does not change a binding into a different type. Use a new let binding for shadowing.
