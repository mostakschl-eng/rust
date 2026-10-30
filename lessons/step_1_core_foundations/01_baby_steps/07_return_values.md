# Lesson 07: Function Return Values (Getting Answers Back)

Welcome to Lesson 07! 

In Lesson 06, we learned how to send data *into* a function (parameters).
Now, we are going to learn how to **get an answer back** from a function!

---

## 1. The Real-Life Analogy: The Calculator

Imagine you are using a pocket calculator:
* You type in `10` and `20`.
* You press the `[+]` button.
* The calculator thinks for a millisecond, and on its screen, it **hands you back the answer: `30`**.

Once you have that `30`, you can do whatever you want with it:
* You can write it on a receipt.
* You can add tax to it.
* You can store it in your wallet.

> In programming:  
> When a function calculates something and gives you back the answer, we say the function **Returns** a value!

---

**Quick preview of `let`:** `let result = add_numbers(10, 20);` binds the returned value to the name `result`, like writing an answer on a labeled card. `=` initializes it and `;` finishes the declaration. Lesson 08 explores variables in detail.

## 2. Returning Values in Rust: The Arrow `->`

Look at this simple, elegant Rust function:

```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // Notice: NO semicolon!
}

fn main() {
    let result = add_numbers(10, 20);
    println!("The answer is: {}", result);
}
```

### The Output on Screen:
```text
The answer is: 30
```

---

## 3. Breaking Down the Magic: What is `-> i32`?

Look at the top line: `fn add_numbers(a: i32, b: i32) -> i32`

| Code Part | What does it mean? |
|---|---|
| `a: i32, b: i32` | The two input numbers (parameters). |
| `->` | The **Return Arrow**. It points to what comes out of the function. |
| `i32` | The **type** of data that will be returned (a whole number). |

The `-> i32` is a promise to Rust:
> *"I promise that when this function finishes, it will hand back a whole number (`i32`)."*

---

## 4. The Famous Rust Secret: Why is there NO Semicolon `;` on `a + b`?

This is one of Rust's coolest features!

In Rust, there is a difference between an **Action** and a **Value**:
* **Statement:** `println!("Hi");` evaluates a printing expression and discards its result. Printing itself produces the unit value `()`.
* **Expression:** `a + b` produces a value. As the final expression of this function body, it becomes the return value.

> **Rust Golden Rule:**  
> A function body's final expression, without a semicolon, supplies its return value; that value must match the declared return type!

You can also write `return a + b;`. A final expression is common at the end of a function; `return` is useful when leaving a function earlier.

---

## 5. More Examples: Multiply and Subtract

```rust
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    let price = multiply(5, 20); // 5 tickets at $20 each
    let discount = subtract(price, 15); // $15 discount coupon

    println!("Original price: ${}", price);
    println!("Price after coupon: ${}", discount);
}
```

### The Output on Screen:
```text
Original price: $100
Price after coupon: $85
```

---

## 6. Common Beginner Traps

### ❌ Trap 1: Accidentally Adding a Semicolon `;` to the Return Line
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b; // <-- OOPS! Added a semicolon!
}
```
* **What happens:** By adding `;`, you turned a value into an empty action statement! 
* **What Rust says:** `error[E0308]: mismatched types: expected 'i32', found '()'`.  
  Rust expected an integer answer, but you gave it empty air `()` because of the semicolon!
* **Fix:** Simply remove the semicolon from the last line!

---

## 7. Summary Checklist
- [x] Use `-> Type` to declare what kind of answer a function gives back.
- [x] Omit the semicolon `;` on the last line to return that value.
- [x] Store returned answers in variables: `let answer = add(5, 5);`.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/01_baby_steps/task_07_function_returns/TASK.md`](../../../tasks/step_1_core_foundations/01_baby_steps/task_07_function_returns/TASK.md)**.
1. Read the task requirements.
2. Build math calculator functions (Area of Rectangle and Perimeter).
3. Run `cargo run` and verify the math calculations!
