# Lesson 10: Variable Shadowing & Constants (`const`)

Welcome to Lesson 10! 

This is the final lesson of **Level 1: Absolute Baby Steps**! 
Today, we will master two very clever Rust concepts:
1. **Constants (`const`):** Values that are permanently carved in stone forever.
2. **Variable Shadowing (`let` again):** Stacking a new sheet of paper on top of an old one.

---

## 1. Constants: Carved in Granite Forever (`const`)

Imagine values in the universe that can **never, ever change**:
* The number of hours in a day: always `24`.
* The speed of light: always `299,792,458` meters/second.
* The freezing temperature of water: always `0` Celsius.

In Rust, you can declare permanent values using the keyword **`const`**:

```rust
const HOURS_IN_DAY: u32 = 24;
const PI_APPROX: f64 = 3.14159;

fn main() {
    println!("Hours in a day: {}", HOURS_IN_DAY);
    println!("Pi: {}", PI_APPROX);
}
```

### The 3 Rules of `const`:
1. **ALL UPPERCASE:** Rust conventions require constants to be written in ALL CAPS with underscores (e.g. `MAX_POINTS`, `TAX_RATE`).
2. **Type is Mandatory:** You **must** specify the type (e.g. `: u32` or `: f64`). Rust will not guess for constants.
3. **Never `mut`:** You can never write `const mut`. Constants are permanently fixed across the entire life of the program.

---

## 2. What is Variable Shadowing?

Imagine you have a piece of paper on your desk with the number `5` written on it.
You take a brand new clean piece of paper, write `10` on it, and **place it directly on top** of the first piece of paper!

When you look down at your desk, what do you see? You only see `10`! 
The second paper has **shadowed** (hidden) the first paper.

In Rust, when you declare a variable with `let` that has the **exact same name** as an earlier variable, the second variable **shadows** the first:

```rust
fn main() {
    let x = 5;
    println!("First x is: {}", x); // Prints 5

    let x = x + 1; // Creates a NEW variable named 'x', shadowing the old one!
    println!("Second x is: {}", x); // Prints 6

    let x = x * 2; // Shadows again!
    println!("Third x is: {}", x); // Prints 12
}
```

### The Output on Screen:
```text
First x is: 5
Second x is: 6
Third x is: 12
```

---

## 3. Why is Shadowing Better than `mut`?

Here is the genius secret of shadowing: **You can change the data type!**

Imagine a user types their age on a website: `"25"` (this is text).
You want to convert it into a number `25` so you can do math.

* **Without shadowing:** You have to invent two ugly names:
  ```rust
  let age_text = "25";
  let age_number = 25; // Awkward!
  ```
* **With shadowing:** You reuse the clean, natural name `age`:
  ```rust
  let age = "25";     // 'age' is text (&str)
  let age = 25;       // 'age' is now a number (i32)! The old text is shadowed!
  ```

---

## 4. Comparing `let`, `let mut`, and `const`

| Feature | `let x = 5;` | `let mut x = 5;` | `const X: i32 = 5;` |
|---|---|---|---|
| Can it change value? | ❌ No | ✅ Yes (`x = 10;`) | ❌ Never |
| Can it change data type? | Only via shadowing | ❌ No | ❌ Never |
| Must you write the type? | Optional | Optional | ✅ Mandatory (`: i32`) |
| Where can it be placed? | Inside functions | Inside functions | Anywhere (even global) |

---

## 5. Summary Checklist of Level 1 Mastery!
🎉 You have completed all 10 lessons of Level 1! You now know:
- [x] How code runs from `fn main()`.
- [x] How to print text and use `{}` placeholders.
- [x] How to define functions, pass parameters, and return values.
- [x] How variables store information in memory (`let`).
- [x] How `mut` unlocks variables when they need to change.
- [x] How `const` carves permanent values, and how `let` shadows older variables.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/01_baby_steps/task_10_constants_shadowing/TASK.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_10_constants_shadowing/TASK.md)**.
1. Read the task requirements.
2. Build a store pricing & tax calculator using `const` and variable shadowing.
3. Run `cargo run` and celebrate finishing Level 1!
