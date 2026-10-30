# Lesson 30: Loop Labels & Nested Loop Control (`'outer: loop`)

Welcome to Lesson 30! 

This is the graduation lesson for **Level 3: Logic, Choices & Loops**!
Today, we master a tool that makes complex algorithms look effortless: **Nested Loops and Loop Labels**!

---

## 1. What are Nested Loops? (A Loop Inside a Loop)

Imagine a digital clock on your desk:
* **The Outer Loop (Hours):** Counts 1, 2, 3... up to 12.
* **The Inner Loop (Minutes):** For *every single hour*, it counts 0 to 59 minutes!

Whenever you work with:
* Grid coordinates (Rows and Columns)
* Chess boards (8 rows x 8 columns)
* Pixel displays (1920 width x 1080 height)

You use a loop inside another loop!

---

## 2. The Classic Problem: The Trapped `break`

In almost every programming language:
* If you write `break;` inside an inner loop, it **only breaks the inner loop**.
* The outer loop continues spinning!
* What if you find what you are looking for and want to **break out of BOTH loops immediately**?

In C or Python, you have to create messy boolean flags (`found = true`) and break multiple times.
In Rust, the creators invented a genius solution: **Loop Labels**!

---

## 3. Rust's Superpower: Named Loop Labels (`'label: loop`)

In Rust, you can give any loop a **custom name** starting with a single quote (like `'outer` or `'grid`):

```rust
fn main() {
    println!("Searching for the target in a 3x3 grid...");

    // Here we label the outer loop:
    'outer_grid: for row in 1..=3 {
        for col in 1..=3 {
            println!("Scanning cell: Row {}, Col {}", row, col);

            if row == 2 && col == 2 {
                println!("🎯 TARGET FOUND at [2, 2]! Breaking out of ALL loops!");
                break 'outer_grid; // Breaks BOTH loops in one shot!
            }
        }
    }

    println!("Search complete!");
}
```

### The Output on Screen:
```text
Searching for the target in a 3x3 grid...
Scanning cell: Row 1, Col 1
Scanning cell: Row 1, Col 2
Scanning cell: Row 1, Col 3
Scanning cell: Row 2, Col 1
Scanning cell: Row 2, Col 2
🎯 TARGET FOUND at [2, 2]! Breaking out of ALL loops!
Search complete!
```

Notice: Row 3 was never even started! `break 'outer_grid;` instantly shut down both the inner and outer loops!

---

## 4. Syntax Rules for Loop Labels
1. Always begins with a single quote: `'label_name:`.
2. Sits directly in front of `loop`, `while`, or `for`:
   ```rust
   'search: while condition { ... }
   'game: loop { ... }
   ```
3. To break out to that specific level:
   ```rust
   break 'search;
   ```
4. You can also use `continue 'search;` to skip to the next lap of the outer loop!

---

## 5. Milestone 3: You Have Mastered Level 3!

🎉 Look at what you have conquered across Lessons 21 to 30:
- [x] Making choices with `if`, `else`, and multi-branch `else if`.
- [x] The 6 comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`).
- [x] Compound boolean logic with `&&`, `||`, and `!`.
- [x] Assigning values directly from `if` expressions (`let x = if ...;`).
- [x] Infinite repeating loops with `loop` and emergency exit `break`.
- [x] Returning calculated values directly from loops: `break answer;`.
- [x] Condition-driven repetition with `while`.
- [x] Safe range iteration with `for i in 1..=10`.
- [x] Multi-level nested loops and targeted exits with `'label: loop`.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/03_logic_and_loops/task_30_loop_labels/TASK.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_30_loop_labels/TASK.md)**.
1. Read the task requirements.
2. Build a grid treasure scanner using nested `for` loops and a labeled `'scanner` loop break.
3. Run `cargo run` and celebrate completing Level 3!
