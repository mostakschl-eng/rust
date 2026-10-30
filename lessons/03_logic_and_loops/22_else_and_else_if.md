# Lesson 22: Alternative Paths (`else` and `else if`)

Welcome to Lesson 22! 

In Lesson 21, our `if` gatekeeper only did something when the condition was `true`. If `false`, it did nothing.
In the real world, you almost always need a **Plan B**:
> *"If it rains, take an umbrella. **Otherwise**, wear sunglasses!"*

In Rust, this Plan B is called **`else`**!

---

## 1. The Fork in the Road: `if` and `else`

Think of walking down a path that splits into two directions:
* **Left Road (`if`):** If you have a passport, go to the International Flight gate.
* **Right Road (`else`):** Otherwise, go to the Domestic Flight gate.

You **must** take exactly one of the two paths. You cannot take both, and you cannot take neither!

```rust
fn main() {
    let age = 16;

    if age >= 18 {
        println!("You are an adult. You can vote! 🗳️");
    } else {
        println!("You are under 18. You cannot vote yet. ⏳");
    }
}
```

### The Output on Screen:
```text
You are under 18. You cannot vote yet. ⏳
```

---

## 2. Multiple Decisions: The `else if` Ladder

What if you have **more than two choices**? 
Like grading school exams:
* Score 90 or more -> **Grade A**
* Score 80 or more -> **Grade B**
* Score 70 or more -> **Grade C**
* Anything lower -> **Needs Improvement**

In Rust, you chain conditions together using **`else if`**:

```rust
fn main() {
    let score = 84;

    if score >= 90 {
        println!("Grade: A 🌟");
    } else if score >= 80 {
        println!("Grade: B 👍");
    } else if score >= 70 {
        println!("Grade: C 👌");
    } else {
        println!("Grade: Needs Improvement 📚");
    }
}
```

### The Output on Screen:
```text
Grade: B 👍
```

---

## 3. The Golden Rule of `else if`: Top-to-Bottom Execution

Rust checks `else if` conditions **from top to bottom, one by one**.
The moment Rust finds the **FIRST condition that is `true`**:
1. It runs that block's code.
2. It **skips all the remaining `else if` and `else` blocks completely**!
3. It exits the decision chain.

Look at why order matters:
```rust
let score = 95;

// If you wrote it in the wrong order:
if score >= 70 {
    println!("Grade: C"); // ❌ OOPS! 95 is >= 70, so Rust stops here and prints C!
} else if score >= 90 {
    println!("Grade: A"); // Never reached!
}
```
* **Lesson:** Always arrange multi-step conditions in logical order (highest to lowest)!

---

## 4. Summary Checklist
- [x] `else { ... }` runs when the `if` condition is `false`.
- [x] `else if condition { ... }` lets you test multiple choices in sequence.
- [x] Rust stops at the first `true` condition and skips the rest.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/03_logic_and_loops/task_22_else_and_else_if/TASK.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_22_else_and_else_if/TASK.md)**.
1. Read the task requirements.
2. Build a movie theater ticket pricing system based on customer age (Child, Adult, Senior).
3. Run `cargo run` and test with different ages!
