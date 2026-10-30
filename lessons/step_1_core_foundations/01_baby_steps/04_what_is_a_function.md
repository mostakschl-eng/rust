# Lesson 04: What is a Function? Reusable Recipes

Welcome to Lesson 04! 

This is **one of the most important concepts in all of programming**. Once you understand functions, you understand how real software is built!

---

## 1. The Real-Life Analogy: The Coffee Machine Button

Imagine you work in a cafe.
Every time a customer asks for a cappuccino:
1. You grind coffee beans.
2. You heat the water.
3. You steam the milk.
4. You pour it into a cup.

Writing down all four steps every single time is exhausting!
Instead, the coffee machine has a single button labeled: **`[Cappuccino]`**.

Whenever a customer orders, you don't re-explain the 4 steps. You simply **press the button**, and the machine automatically does all 4 steps for you.

> In programming:
> A **Function** is your custom button or recipe card.  
> You give it a name, put instructions inside it, and whenever you need those instructions, you just call its name!

---

## 2. Defining a Function vs Calling a Function

There are **two separate stages** to every function:

1. **Creating the function (Defining the recipe):** Writing down what instructions belong to this name.
2. **Calling the function (Pressing the button):** Telling Rust to actually run those instructions now.

Let's look at working Rust code:

```rust
// 1. Here we CREATE the function
fn make_coffee() {
    println!("1. Grinding coffee beans...");
    println!("2. Steaming fresh milk...");
    println!("3. Your hot coffee is ready!");
}

// 2. Main is where our program starts
fn main() {
    println!("Customer 1 arrives:");
    make_coffee(); // <-- Here we CALL the function!

    println!("Customer 2 arrives:");
    make_coffee(); // <-- Call it again! No need to rewrite the 3 steps!
}
```

### The Output on Screen:
```text
Customer 1 arrives:
1. Grinding coffee beans...
2. Steaming fresh milk...
3. Your hot coffee is ready!
Customer 2 arrives:
1. Grinding coffee beans...
2. Steaming fresh milk...
3. Your hot coffee is ready!
```

---

## 3. Understanding Every Piece of the Code

| Code Part | What does it mean? |
|---|---|
| `fn` | Keyword telling Rust: *"I am declaring a new function."* |
| `make_coffee` | The **name** you gave to this function. You can name it whatever you like (use lowercase with underscores). |
| `()` | Parentheses. Shows that this is a function. |
| `{ ... }` | Curly braces containing all the steps this function performs. |
| `make_coffee();` | The **call**. The parentheses `()` and semicolon `;` tell Rust: *"Go run `make_coffee` right now!"* |

---

## 4. Why Do We Need Functions? (The "Deep Why")

Imagine your app prints a beautiful header bar on 10 different screens.
* **Without functions:** You copy and paste 10 lines of code in 10 different places (100 lines total). If you want to change the title, you have to edit 10 places!
* **With a function:** You write `fn print_header() { ... }` once. In the other 10 places, you just write `print_header();`. If you want to change it, you only edit **one single place**!

---

## 5. Common Beginner Traps

### ❌ Trap 1: Forgetting the Parentheses `()` When Calling
```rust
fn make_coffee() {
    println!("Coffee ready!");
}

fn main() {
    make_coffee; // <-- Forgot ()!
}
```
* **What happens:** With `make_coffee` defined as above, this refers to the function without calling it; the compiler may warn that the path statement has no effect. Use `make_coffee();` to run its body.

---

## 6. Summary Checklist
- [x] A function is a named, reusable block of code.
- [x] Creating a function starts with `fn function_name() { ... }`.
- [x] Calling a function is written as `function_name();`.
- [x] Functions save time and keep your code clean and DRY (*Don't Repeat Yourself*).

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/01_baby_steps/task_04_first_function/TASK.md`](../../../tasks/step_1_core_foundations/01_baby_steps/task_04_first_function/TASK.md)**.
1. Read the task requirements.
2. Define your own custom reusable greeting function and call it multiple times.
3. Run `cargo run` to see your function in action!
