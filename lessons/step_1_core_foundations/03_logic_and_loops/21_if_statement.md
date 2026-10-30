# Lesson 21: Making Decisions: The `if` Statement

Welcome to **Level 3: Logic, Choices & Loops**! 

Up until now, our programs ran like a straight road: line 1, then line 2, then line 3.
In this level, we teach our programs how to **think and make smart choices**!

---

## 1. The Real-Life Analogy: The Rollercoaster Height Gate

Imagine you are standing at the entrance of a giant rollercoaster at an amusement park:
* There is a measuring stick at the gate: **"Must be 48 inches or taller to ride"**.
* If you are 50 inches tall: **The gate swings open and you enter!**
* If you are 40 inches tall: **The gate stays shut.**

In programming:
> An **`if` Statement** is a gatekeeper.  
> It checks a condition. If the condition is **`true`**, it opens the door `{ }` and executes the code inside! If `false`, it skips the code entirely.

---

## 2. Your First `if` Statement in Rust

Look at how clean Rust conditional code looks:

```rust
fn main() {
    let rider_height_inches = 52;

    println!("Approaching the rollercoaster gate...");

    // Here is our decision gatekeeper:
    if rider_height_inches >= 48 {
        println!("Access Granted! Enjoy the ride! 🎢");
    }

    println!("Gate check complete.");
}
```

### The Output on Screen:
```text
Approaching the rollercoaster gate...
Access Granted! Enjoy the ride! 🎢
Gate check complete.
```

If `rider_height_inches` was `40`, the middle message would never print!

---

## 3. Two Special Rust Rules About `if`

### Rule 1: No Parentheses Needed!
In languages like C, Java, or JavaScript, you have to write `if (age >= 18)`.  
In Rust, parentheses are **not needed**! You just write:
```rust
if age >= 18 { ... }
```

### Rule 2: Curly Braces `{ }` Are Mandatory!
In some languages, you can skip `{}` if you only have one line. In Rust, curly braces are **always 100% required**. This prevents accidental bugs!

---

## 4. The Strict Truth: Rust Has No "Truthy" or "Falsy" Values!

In JavaScript or Python, programmers sometimes write:
```text
if 1 { ... } // Or if "hello" { ... }
```
In Rust, this is **completely forbidden**!
> **The condition inside `if` MUST be an exact Boolean (`true` or `false`)!**

If you try to pass a number or text, Rust will immediately stop you:
`error[E0308]: mismatched types: expected 'bool', found integer`.

---

## 5. Summary Checklist
- [x] `if condition { ... }` runs code only when condition is `true`.
- [x] No parentheses are needed around the condition.
- [x] Curly braces `{ }` are always mandatory.
- [x] The condition must strictly evaluate to a `bool` (`true` or `false`).

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/03_logic_and_loops/task_21_if_statement/TASK.md`](../../../tasks/step_1_core_foundations/03_logic_and_loops/task_21_if_statement/TASK.md)**.
1. Read the task requirements.
2. Build an ATM cash withdrawal validator that only dispenses cash if account balance is sufficient.
3. Run `cargo run` and test with different balances!

## Symbol reference

| Symbol | Meaning |
|---|---|
| `if` | Evaluate a bool and conditionally run a block |
| `>=` | Greater than or equal comparison |

## Common pitfalls and limits

Use a boolean condition. This small ATM example assumes a positive requested amount; it demonstrates a branch rather than a complete banking system.
