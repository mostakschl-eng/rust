# Why this Study Tracker checkpoint works

[Back to the mission](../TASK.md)

## Everyday picture

Think of placing one 25-minute token in a jar after each session. The loop repeats that action three times. A goal sign tells you whether the jar is below, at, or above 90. Rust stores a number here, not tokens, and the category is a comparison result rather than a measurement of learning.

## Step by step

The total starts at zero. Each loop turn adds 25, so the accumulated values are 25, 50, 75. The range `1..=3` includes 3. The checks after the loop confirm the actual accumulation. The progress function checks its conditions in order; only the first matching branch supplies its result. The codes 0, 1, 2 are our convention, not Rust keywords. Numeric codes let us practise functions now; enums provide clearer named categories in later lessons. The remaining-time function prevents a negative display after reaching the goal. Its assertions cover both equality and above-goal behaviour.

The [checking guide](../../../../../guides/testing_basics.md) introduces the assertion macro. The exclamation mark marks a macro call. Successful checks print nothing; failed checks panic. Assertions check these chosen cases, not every possible program behaviour.

## Symbols

| Symbol | Meaning here |
|---|---|
| `let mut` | Bind a value that can be changed |
| `for ... in` | Visit each item in a range |
| `..=` | Include the ending number |
| `+=` | Add to the current value and store the result |
| `<`, `==` | Less-than and equality comparisons |
| `if / else if / else` | Choose one branch |
| `assert_eq!` | Check the actual answer against an expected answer |

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

// Codes: 0 below, 1 exactly at, 2 above the goal.
fn progress_level(total: i32, goal: i32) -> i32 {
    if total < goal {
        0
    } else if total == goal {
        1
    } else {
        2
    }
}

// shortcut: fixed small nonnegative inputs, add validation when accepting user data.
fn remaining_minutes(total: i32, goal: i32) -> i32 {
    if total < goal {
        goal - total
    } else {
        0
    }
}

fn main() {
    assert_eq!(progress_level(0, DAILY_GOAL), 0);
    assert_eq!(progress_level(89, DAILY_GOAL), 0);
    assert_eq!(progress_level(90, DAILY_GOAL), 1);
    assert_eq!(progress_level(91, DAILY_GOAL), 2);
    assert_eq!(remaining_minutes(0, DAILY_GOAL), 90);
    assert_eq!(remaining_minutes(75, DAILY_GOAL), 15);
    assert_eq!(remaining_minutes(90, DAILY_GOAL), 0);
    assert_eq!(remaining_minutes(91, DAILY_GOAL), 0);
    let sessions = 3;
    let minutes_each = 25;
    let mut total = 0;
    let name = "Rahim";
    print_header();
    println!("Learner: {}", name);
    println!("Daily goal: {} minutes", DAILY_GOAL);
    println!("Sessions: {} x {} minutes", sessions, minutes_each);
    for session in 1..=sessions {
        total += minutes_each;
        println!("Session {}: {} minutes", session, minutes_each);
    }
    assert_eq!(total, 75);
    println!("Studied: {} minutes", total);
    println!("Remaining: {} minutes", remaining_minutes(total, DAILY_GOAL));
    let level = progress_level(total, DAILY_GOAL);
    if level == 0 {
        println!("Progress: below goal");
    } else if level == 1 {
        println!("Progress: goal reached");
    } else {
        println!("Progress: above goal");
    }
    print_footer();
}
```

## Observed stdout

```text
=== Study Tracker ===
Learner: Rahim
Daily goal: 90 minutes
Sessions: 3 x 25 minutes
Session 1: 25 minutes
Session 2: 25 minutes
Session 3: 25 minutes
Studied: 75 minutes
Remaining: 15 minutes
Progress: below goal
Keep learning, one session at a time.
```

The reference checks the cases requested in TASK.md before finishing the report. Compiler messages are separate from stdout.

## Common pitfalls and limits

- `1..3` runs two turns, not three. Resetting total inside the loop loses earlier sessions.
- `=` assigns; `==` compares. Use `mut` for an accumulating total.
- Above-goal remaining time is zero here. A negative shortfall is not the display requirement.
- Check equality separately. A below/above split alone can mislabel exactly 90.
- These fixed small inputs avoid integer overflow. This is not a general input-validation system, and these checks do not prove every possible input correct.

## Source checks

[Rust Book: control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html), [standard-library assert_eq!](https://doc.rust-lang.org/std/macro.assert_eq.html). These references describe the language constructs used here. The exact output above comes from running this checkpoint, not from a promise about all future versions of the app.
