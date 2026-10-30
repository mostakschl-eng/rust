# Lesson 15: Math in Rust (Operators: `+`, `-`, `*`, `/`, `%`)

Welcome to Lesson 15! 

Computers are the fastest calculators in the universe. Today, we will learn how to perform all standard arithmetic in Rust, including the secret weapon of programmers: the **Modulus `%` (Remainder) operator**!

---

## 1. The 5 Core Arithmetic Operators

| Symbol | Name | Example | Output | What does it do? |
|---|---|---|---|---|
| `+` | Addition | `10 + 5` | `15` | Adds two numbers together |
| `-` | Subtraction | `10 - 5` | `5` | Subtracts the right number from the left |
| `*` | Multiplication | `10 * 5` | `50` | Multiplies two numbers |
| `/` | Division | `10 / 5` | `2` | Divides left by right |
| **`%`** | **Modulus (Remainder)** | `10 % 3` | **`1`** | **Gives the leftover remainder after division!** |

---

## 2. The Superpower of the Modulus `%` (Remainder)

Many beginners ask: *"Why do we need `%`?"*

Imagine you have **10 chocolate cookies** and you want to share them equally among **3 friends**:
* Each friend gets **3 cookies** (`3 * 3 = 9 cookies distributed`).
* How many cookies are left over in your hand? **1 cookie**!
* In Rust: `10 % 3` equals **`1`**!

### Real-World Use Cases for `%`:
1. **Checking Even vs Odd:**
   * Any number `% 2 == 0` is an **Even number** (e.g. `8 % 2 = 0`).
   * Any number `% 2 != 0` is an **Odd number** (e.g. `7 % 2 = 1`).
2. **Clock Math:**
   * If it is 10:00 AM and 5 hours pass: `(10 + 5) % 12` = **`3:00 PM`**!

---

## 3. Order of Operations: PEMDAS

Just like school mathematics, Rust evaluates math in this exact priority:
1. **Parentheses `( )`** come first!
2. **Multiplication `*`, Division `/`, and Modulus `%`** come next.
3. **Addition `+` and Subtraction `-`** come last.

```rust
fn main() {
    let wrong = 10 + 5 * 2;   // 5 * 2 is done first! (10 + 10 = 20)
    let right = (10 + 5) * 2; // Parentheses first!   (15 * 2 = 30)

    println!("Without parentheses: {}", wrong);
    println!("With parentheses:    {}", right);
}
```

---

## 4. Code Example: Full Math Operations

```rust
fn main() {
    let a = 20;
    let b = 6;

    println!("Addition:       {} + {} = {}", a, b, a + b);
    println!("Subtraction:    {} - {} = {}", a, b, a - b);
    println!("Multiplication: {} * {} = {}", a, b, a * b);
    println!("Division:       {} / {} = {}", a, b, a / b);
    println!("Remainder:      {} % {} = {}", a, b, a % b);
}
```

### The Output on Screen:
```text
Addition:       20 + 6 = 26
Subtraction:    20 - 6 = 14
Multiplication: 20 * 6 = 120
Division:       20 / 6 = 3
Remainder:      20 % 6 = 2
```
*(Notice: `20 / 6` is `3`, and the leftover remainder `20 % 6` is `2`. `3 * 6 + 2 = 20`!)*

---

## 5. Summary Checklist
- [x] Standard math: `+`, `-`, `*`, `/`.
- [x] `%` computes the leftover remainder.
- [x] Use parentheses `( )` to control which math runs first.
- [x] Integer division throws away fractions; float division keeps them.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_15_math_operations/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_15_math_operations/TASK.md)**.
1. Read the task requirements.
2. Build an egg packaging calculator (total dozens of eggs and leftover single eggs using `/` and `%`).
3. Run `cargo run` and test your math!
