# Lesson 48: The `match` Expression: Exhaustive Matching

Now that our enums can hold rich data, how do we inspect which variant is active and extract its cargo?  
We use Rust's supreme control flow weapon: the **`match` expression**! 🎯

---

## 1. The Real-Life Analogy: The Coin Sorter Machine 🪙

Think of an automated coin sorting machine at a bank:
* You dump a handful of mixed coins into the top hopper.
* The machine inspects each coin:
  * If it's a **Penny** -> It rolls down Track 1 and adds 1 cent.
  * If it's a **Nickel** -> It rolls down Track 2 and adds 5 cents.
  * If it's a **Dime** -> It rolls down Track 3 and adds 10 cents.
  * If it's a **Quarter** -> It rolls down Track 4 and adds 25 cents.
* **Every single coin is accounted for**; no coin gets stuck in limbo.

In Rust:
> A **`match` expression** compares a value against a series of **patterns** and executes the code block of the first matching arm.  
> It also extracts any data packaged inside the variant!

---

## 2. Exhaustiveness: Rust's Sacred Shield 🛡️

Here is the most important rule about `match` in Rust:
> **`match` MUST BE EXHAUSTIVE!**  
> You **must** cover every single possible scenario that can happen. If you miss even one variant, your program **refuses to compile**!

Look at what happens if you forget a variant:
```rust
enum Direction { North, South, East, West }

fn navigate(d: Direction) {
    match d {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        // Forgot East and West!
    }
}
```
Rust compiler halts you:
```text
error[E0004]: non-exhaustive patterns: `Direction::East` and `Direction::West` not covered
```
Think about how many production crashes in other languages happen because a developer added a new feature and forgot to update one `switch` statement. In Rust, the compiler makes this bug **physically impossible**!

---

## 3. Extracting Data from Enums with `match`

Look at how easily `match` unwraps the payload stored inside each variant:

```rust
#[derive(Debug)]
enum PlayerAction {
    Idle,
    Walk { direction: String, speed: u32 },
    Jump(f64), // Jump height in meters
    Speak(String),
}

fn process_action(action: PlayerAction) {
    match action {
        PlayerAction::Idle => {
            println!("Player is standing still.");
        }
        PlayerAction::Walk { direction, speed } => {
            println!("Walking {} at {} km/h.", direction, speed);
        }
        PlayerAction::Jump(height) => {
            println!("Jumped {:.1} meters high!", height);
        }
        PlayerAction::Speak(message) => {
            println!("Player says: \"{}\"", message);
        }
    }
}

fn main() {
    process_action(PlayerAction::Jump(1.8));
    process_action(PlayerAction::Speak(String::from("Ready for battle!")));
    process_action(PlayerAction::Walk {
        direction: String::from("North"),
        speed: 5,
    });
}
```

### 📋 Expected Terminal Output:
```text
Jumped 1.8 meters high!
Player says: "Ready for battle!"
Walking North at 5 km/h.
```

---

## 4. The Catch-All Pattern: `_`

What if you have an enum with 20 variants or an integer with millions of possible numbers, and you only care about 2 specific cases?  
Use the wildcard **`_`** as the final default fallback arm:

```rust
fn roll_dice(number: u8) {
    match number {
        6 => println!("JACKPOT! You rolled a 6! 🎲"),
        1 => println!("Bummer! You rolled a 1! 😢"),
        _ => println!("You rolled a regular number: {}", number),
    }
}

fn main() {
    roll_dice(6);
    roll_dice(3);
}
```

---

## 5. Returning Values Directly from `match`

Just like `if`, `match` is an **expression**, meaning it can compute and return a value:

```rust
enum Status { Success, NotFound, ServerError }

fn get_status_code(status: Status) -> u16 {
    match status {
        Status::Success => 200,
        Status::NotFound => 404,
        Status::ServerError => 500,
    }
}
```

---

## 6. Quick Recap Checklist
- [x] `match` pattern matches against values and variants.
- [x] `match` is **exhaustive** — every possible case must be covered.
- [x] Extracts attached payload data seamlessly.
- [x] Use `_` as a catch-all pattern for remaining cases.
- [x] `match` can return values to initialize variables.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/05_structs_and_enums/task_48_match_exhaustive/TASK.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_48_match_exhaustive/TASK.md).
* Next lesson: [Lesson 49: Match Guards (Adding `if` inside match arms)](file:///d:/rust-learning/lessons/05_structs_and_enums/49_match_guards.md).
