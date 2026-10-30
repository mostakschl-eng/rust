# Lesson 24: Logical Operators (AND `&&`, OR `||`, NOT `!`)

Welcome to Lesson 24! 

In real life, decisions are rarely based on just one single question.
Usually, multiple conditions must be checked at the same time:
* *"Do you have a driver's license **AND** are you wearing a seatbelt?"*
* *"Do you have a Student ID **OR** are you under 12 years old?"*

In Rust, we combine conditions using **Logical Operators**!

---

## 1. The 3 Logical Operators in Rust

| Operator | Name | Meaning | Truth Rule |
|---|---|---|---|
| **`&&`** | **Logical AND** | Both sides must be true | Returns `true` **ONLY if BOTH A and B are true**. |
| **`\|\|`** | **Logical OR** | At least one side must be true | Returns `true` **if EITHER A is true OR B is true**. |
| **`!`** | **Logical NOT** | Flips true to false | Inverts the boolean (`!true` is `false`). |

---

## 2. Logical AND (`&&`): The Two-Key Safe

Imagine a bank vault that requires **two keys turned at the same time**:
* Key A (Manager) = `true`
* Key B (Security Guard) = `true`
* `Key A && Key B` -> Vault opens!

If either person is missing, `&&` evaluates to `false` and the door stays shut:

```rust
fn main() {
    let has_license = true;
    let is_sober = true;

    if has_license && is_sober {
        println!("You can drive safely! 🚗");
    } else {
        println!("You cannot drive! 🛑");
    }
}
```

---

## 3. Logical OR (`||`): The Multiple Payment Options

Imagine buying a coffee:
* You can pay with **Cash** (`cash_available`)
* **OR** you can pay with **Credit Card** (`card_available`)

As long as you have **at least one** of them, you can buy the coffee:

```rust
fn main() {
    let has_cash = false;
    let has_card = true;

    if has_cash || has_card {
        println!("Payment accepted! ☕");
    } else {
        println!("Payment declined: Need cash or card! ❌");
    }
}
```

---

## 4. Short-Circuit Evaluation (Rust's Speed Optimization)

Rust is famous for speed, and it is smart about logic:

1. **In `A && B`:** If `A` is `false`, Rust doesn't even bother looking at `B`. It immediately knows the whole thing is `false`!
2. **In `A || B`:** If `A` is `true`, Rust doesn't evaluate `B`. It already knows the answer is `true`!

The right-hand expression is skipped entirely, including any side effects. Order can matter for correctness, such as checking a divisor is nonzero before division.

---

## 5. Summary Checklist
- [x] `&&` requires both sides to be `true`.
- [x] `||` requires at least one side to be `true`.
- [x] `!` inverts any boolean condition.
- [x] Short-circuit evaluation skips unnecessary checks.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/03_logic_and_loops/task_24_logical_operators/TASK.md`](../../../tasks/step_1_core_foundations/03_logic_and_loops/task_24_logical_operators/TASK.md)**.
1. Read the task requirements.
2. Build an airport boarding gate verification check using both `&&` and `||`.
3. Run `cargo run` and test your compound logic!

## Common pitfalls and limits

OR includes the case where both inputs are true. Parentheses control grouping, but evaluation still begins at the left and may skip a group entirely.

## Expected output of the two examples

The AND example prints:

```text
You can drive safely! 🚗
```

The OR example prints:

```text
Payment accepted! ☕
```
