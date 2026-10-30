# Lesson 23: Comparison Operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)

Welcome to Lesson 23! 

In Lesson 21 and 22, we used symbols like `>=` to test numbers.
Today, we examine the **full set of 6 Comparison Operators** that let your program compare values with compatible types and the required comparison support!

---

## 1. The Real-Life Analogy: The Judge's Scale

Imagine a courtroom judge holding a balancing scale:
* You place object A on the left and object B on the right.
* You ask the judge a question: *"Is A heavier than B?"* or *"Are A and B identical?"*
* The judge only gives you one of two answers: **`true`** or **`false`**!

Every comparison operator in Rust compares two values and **hands you back a clean `bool`**.

---

## 2. The 6 Comparison Operators

| Operator | Plain English Meaning | Example Code | Evaluates To |
|---|---|---|---|
| **`==`** | **Is Equal To?** *(Notice: Two equals signs!)* | `5 == 5` | `true` |
| **`!=`** | **Is NOT Equal To?** | `5 != 3` | `true` |
| **`>`** | **Is Greater Than?** | `10 > 2` | `true` |
| **`<`** | **Is Less Than?** | `4 < 9` | `true` |
| **`>=`** | **Is Greater Than or Equal To?** | `5 >= 5` | `true` |
| **`<=`** | **Is Less Than or Equal To?** | `3 <= 5` | `true` |

---

## 3. The #1 Beginner Trap: `=` vs `==`

This is the most common mistake made by new developers across the globe:

* **Single `=` is an Assignment Command:**  
  `let x = 10;` *(Tells Rust: "Drop 10 into the box named x.")*
* **Double `==` is an Equality Question:**  
  `x == 10` *(Asks Rust: "Does the box x currently hold 10?")*

In many other languages (like C or JavaScript), if you accidentally write `if x = 10`, it overwrites the variable and causes a disastrous silent bug!

> **Rust's Protection:**  
> In Rust, if you accidentally write `if x = 10`, the compiler halts immediately:  
> `error: mismatched types: expected 'bool', found '()'`. Rust saves you from yourself!

---

## 4. Code Example: Comparing Numbers and Text

```rust
fn main() {
    let target_score = 100;
    let player_score = 100;
    let high_score = 150;

    let is_exact_match = player_score == target_score;
    let is_new_record = player_score > high_score;
    let is_different = player_score != 0;

    println!("Did player hit target? {}", is_exact_match); // true
    println!("Is it a new record?    {}", is_new_record);   // false
    println!("Score is not zero?     {}", is_different);    // true

    // You can also compare text strings!
    let entered_pin = "1234";
    let correct_pin = "1234";
    if entered_pin == correct_pin {
        println!("PIN Verified! Vault unlocked. 🔓");
    }
}
```

### The Output on Screen:
```text
Did player hit target? true
Is it a new record?    false
Score is not zero?     true
PIN Verified! Vault unlocked. 🔓
```

---

## 5. Summary Checklist
- [x] Single `=` stores a value; double `==` tests for equality.
- [x] `!=` tests if two values are different.
- [x] `>`, `<`, `>=`, `<=` compare sizes.
- [x] All comparison operators return a `bool` (`true` or `false`).

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/03_logic_and_loops/task_23_comparison_operators/TASK.md`](../../../tasks/step_1_core_foundations/03_logic_and_loops/task_23_comparison_operators/TASK.md)**.
1. Read the task requirements.
2. Build an automated login & security pin validator testing equality and threshold limits.
3. Run `cargo run` and test all comparison operators!

## Common pitfalls and limits

Values must support the chosen comparison. Float equality can be surprising after approximate calculations, and NaN is not equal to itself.
