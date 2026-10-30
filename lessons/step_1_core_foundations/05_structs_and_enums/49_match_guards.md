# Lesson 49: Match Guards (Adding `if` Inside Match Arms)

In Lesson 48, we matched patterns by their structural shape:
```rust
Temperature::Celsius(val) => ...
```
What if you want to match **both the shape of the data AND a specific runtime condition**?
For example:
* Match Celsius **only if** `val < 0` (Freezing!).
* Match Celsius **only if** `val >= 100` (Boiling!).
* Match Celsius **otherwise** (Normal room temperature).

In Rust, this superpower is called a **Match Guard**! 🛡️

---

## 1. The Real-Life Analogy: The VIP Nightclub Bouncer 🎟️

Imagine you are standing at the velvet rope of an exclusive nightclub:
* The bouncer asks for your **Ticket** (Pattern matching).
* But even if you have a VIP ticket, the bouncer adds a secondary condition:
  * *"You have a VIP ticket, **IF** you are 21 or older, enter the VIP lounge!"*
  * *"You have a VIP ticket, **IF** you are under 21, enter the alcohol-free lounge!"*

In Rust:
> A **Match Guard** is an extra `if <condition>` placed after a pattern in a `match` arm.  
> The arm will only execute if **BOTH** the pattern matches **AND** the `if` condition evaluates to `true`!

---

## 2. Match Guards in Action

Look at how natural and readable match guards look:

```rust
#[derive(Debug)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn describe_weather(temp: Temperature) {
    match temp {
        // Match Guard: Celsius where degrees are below zero:
        Temperature::Celsius(c) if c < 0 => {
            println!("Freezing cold! ({c}°C) Bundle up! ❄️");
        }
        // Match Guard: Celsius where degrees are boiling:
        Temperature::Celsius(c) if c >= 100 => {
            println!("Boiling hot! ({c}°C) Water turns to steam! ♨️");
        }
        // Fallback for regular Celsius:
        Temperature::Celsius(c) => {
            println!("Mild weather: {c}°C. Enjoy your day! ☀️");
        }
        // Catch-all for Fahrenheit:
        Temperature::Fahrenheit(f) => {
            println!("Fahrenheit reading: {f}°F");
        }
    }
}

fn main() {
    describe_weather(Temperature::Celsius(-10));
    describe_weather(Temperature::Celsius(25));
    describe_weather(Temperature::Celsius(105));
    describe_weather(Temperature::Fahrenheit(72));
}
```

### 📋 Expected Terminal Output:
```text
Freezing cold! (-10°C) Bundle up! ❄️
Mild weather: 25°C. Enjoy your day! ☀️
Boiling hot! (105°C) Water turns to steam! ♨️
Fahrenheit reading: 72°F
```

---

## 3. Combining Outer Variables with Match Guards

Match guards can also compare values against other independent variables outside the match expression:

```rust
fn check_access(user_level: u32, is_admin_override: bool) {
    match user_level {
        level if is_admin_override => {
            println!("Access Granted via Admin Override! (Level {level}) 🔑");
        }
        level if level >= 5 => {
            println!("Access Granted! Clearance Level {level} approved.");
        }
        level => {
            println!("Access Denied. Level {level} insufficient. ⛔");
        }
    }
}

fn main() {
    check_access(2, true);  // Level 2, but has override!
    check_access(7, false); // Level 7, high clearance!
    check_access(3, false); // Level 3, rejected.
}
```

### 📋 Expected Terminal Output:
```text
Access Granted via Admin Override! (Level 2) 🔑
Access Granted! Clearance Level 7 approved.
Access Denied. Level 3 insufficient. ⛔
```

---

## 4. Match Guards and Exhaustiveness ⚠️

Because the compiler cannot predict at compile time whether runtime `if` conditions will always evaluate to `true`, **a match arm with a guard does not count towards covering all patterns**!

You must always provide an unguarded fallback pattern or `_`:
```rust
match x {
    Some(n) if n > 0 => ...,
    Some(n) => ..., // Unguarded fallback for non-positive numbers!
    None => ...,
}
```

---

## 5. Quick Recap Checklist
- [x] Match Guards add an `if condition` directly after the pattern.
- [x] Syntax: `Pattern if condition => { ... }`.
- [x] Allows fine-grained filtering on values extracted from enum variants.
- [x] Can compare extracted variables against external scope variables.
- [x] Requires a fallback arm to ensure exhaustiveness.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/05_structs_and_enums/task_49_match_guards/TASK.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_49_match_guards/TASK.md).
* Next lesson: [Lesson 50: Concise Matching: `if let` and `let else`](file:///d:/rust-learning/lessons/05_structs_and_enums/50_if_let_and_let_else.md).
