# Why this Study Tracker checkpoint works

[Back to the mission](../TASK.md)

## Everyday picture

Think of three 25-minute blocks on a timetable. Multiplication counts their total time. Subtraction finds the gap before the 90-minute target. This analogy describes these fixed durations; the program does not measure a real clock or verify that someone studied.

## Step by step

The two function parameters receive the count and duration. Their product is the returned value. A final expression without a semicolon supplies the function result. In main, the returned 75 is stored as total, and 90 minus 75 becomes 15. The checks run before the report. A successful assertion prints nothing. A failed assertion panics; this is a programming check, not graceful handling of user input. Zero checks exercise the same function with different small inputs.

The [checking guide](../../../../../guides/testing_basics.md) introduces the assertion macro. The exclamation mark marks a macro call. Successful checks print nothing; failed checks panic. Assertions check these chosen cases, not every possible program behaviour.

## Symbols

| Symbol | Meaning here |
|---|---|
| `*` | Multiply session count by duration |
| `-` | Subtract the studied total from the goal |
| `-> i32` | Function returns an i32 number |
| `{}` | Print the corresponding argument |
| `assert_eq!(actual, expected)` | Check equality; panic if unequal |
| `!` | Macro invocation, also seen in println! |

## Complete reference example

This is the complete program stored in solution.rs, exposed through the task manifest as the solution example. From this task folder run `cargo run --example solution`. You can compare without replacing your src/main.rs.

```rust
const DAILY_GOAL: i32 = 90;

fn print_header() {
    println!("=== Study Tracker ===");
}

fn print_footer() {
    println!("Keep learning, one session at a time.");
}

// shortcut: fixed small nonnegative inputs, add validation when accepting user data.
fn study_minutes(sessions: i32, minutes_each: i32) -> i32 {
    sessions * minutes_each
}

fn main() {
    let sessions = 3;
    let minutes_each = 25;
    let total = study_minutes(sessions, minutes_each);
    let remaining = DAILY_GOAL - total;
    assert_eq!(total, 75);
    assert_eq!(remaining, 15);
    assert_eq!(study_minutes(0, 25), 0);
    assert_eq!(study_minutes(3, 0), 0);
    let name = "Rahim";
    print_header();
    println!("Learner: {}", name);
    println!("Daily goal: {} minutes", DAILY_GOAL);
    println!("Sessions: {} x {} minutes", sessions, minutes_each);
    println!("Studied: {} minutes", total);
    println!("Remaining: {} minutes", remaining);
    print_footer();
}
```

## Observed stdout

```text
=== Study Tracker ===
Learner: Rahim
Daily goal: 90 minutes
Sessions: 3 x 25 minutes
Studied: 75 minutes
Remaining: 15 minutes
Keep learning, one session at a time.
```

The reference checks the cases requested in TASK.md before finishing the report. Compiler messages are separate from stdout.

## Common pitfalls and limits

- `3 + 25` counts neither the three durations nor their product.
- A semicolon after the return expression discards that value and breaks this return type.
- Typing75 directly into the report hides a broken calculation.
- The fixed demo uses exact whole minutes, not floating-point values. Arbitrary negative or very large values are outside its input contract. i32 arithmetic can overflow; these few small cases do not prove correctness for every integer.
- An assertion checks only the cases supplied. It does not prove the whole program correct.

## Source checks

[Rust Book: functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html), [standard-library assert_eq!](https://doc.rust-lang.org/std/macro.assert_eq.html). These references describe the language constructs used here. The exact output above comes from running this checkpoint, not from a promise about all future versions of the app.
