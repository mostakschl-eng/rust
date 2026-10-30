# Lesson 13: True or False (Booleans: `bool`)

Welcome to Lesson 13! 

Computers at their lowest level are built of billions of tiny electric switches that are either **ON** or **OFF**.
In programming, we represent this with a data type named **Boolean** (or **`bool`** in Rust).

---

## 1. The Real-Life Analogy: The Light Switch

Think of the light switch in your bedroom:
* It can be flipped **UP** (The light is ON).
* It can be flipped **DOWN** (The light is OFF).
* It can never be "halfway on" or "kinda purple". It is strictly one or the other!

In Rust, a `bool` has only two possible values in the entire universe:
* **`true`** (Yes, On, Enabled)
* **`false`** (No, Off, Disabled)

---

## 2. Declaring Booleans in Rust

Look at how clean and simple booleans look in Rust:

```rust
fn main() {
    let is_logged_in: bool = true;
    let has_subscription: bool = false;
    let is_game_over: bool = false;

    println!("User Logged In?     {}", is_logged_in);
    println!("Has Subscription?   {}", has_subscription);
    println!("Is Game Over?       {}", is_game_over);
}
```

### The Output on Screen:
```text
User Logged In?     true
Has Subscription?   false
Is Game Over?       false
```

---

## 3. The NOT Operator: Flipping the Switch (`!`)

What if you want to flip a boolean to its opposite?
In Rust, the exclamation mark `!` placed in front of a boolean means **NOT** (opposite):

```rust
fn main() {
    let door_locked = true;
    let door_unlocked = !door_locked; // Opposite of true is false!

    println!("Door locked:   {}", door_locked);
    println!("Door unlocked: {}", door_unlocked);
}
```

### The Output on Screen:
```text
Door locked:   true
Door unlocked: false
```

* `!true` becomes `false`.
* `!false` becomes `true`.

---

## 4. Why Are Booleans So Important?

Every video game, social media app, and online banking app runs on booleans:
* `is_password_correct` -> If `true`, let them in. If `false`, block them!
* `has_enough_balance` -> If `true`, approve transaction. If `false`, decline!
* `player_health <= 0` -> Turns into `true` when player dies!

In Level 3, we will use booleans with `if` and `else` to teach our computer how to make real decisions!

---

## 5. Summary Checklist
- [x] A boolean in Rust is written as `bool`.
- [x] It can only ever be `true` or `false`.
- [x] `!` is the NOT operator that flips a boolean to its opposite.
- [x] Booleans are the foundation of computer decision-making.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_13_boolean_flags/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_13_boolean_flags/TASK.md)**.
1. Read the task requirements.
2. Build an account security status checker using boolean flags and the `!` operator.
3. Run `cargo run` and test your boolean flags!
