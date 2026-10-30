# Why this Study Tracker checkpoint works

[Back to the mission](../TASK.md)

## Everyday picture

Think of a paper study card. The name is a label on the card. The daily goal is a printed rule. The header and footer are reusable stamps. A variable binds a name to a value; Rust does not literally store a paper card or a stamp.

## Step by step

The constant supplies the goal everywhere it is used. Its explicit type is required for a constant declaration. The variable `name` holds the text literal. `main` calls the header, prints two lines, and calls the footer. Defining a function alone does not execute it. Reusable means another call can run the same body again.

## Symbols

| Symbol | Meaning here |
|---|---|
| `const` | Declare a constant value with an explicit type |
| `: i32` | Signed 32-bit whole-number type |
| `let` | Bind a name to a value |
| `fn` and `()` | Define a function; parentheses also mark its call |
| `{}` | Function body, or a value placeholder inside a print string |
| `;` | End these statements |

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

fn main() {
    let name = "Rahim";
    print_header();
    println!("Learner: {}", name);
    println!("Daily goal: {} minutes", DAILY_GOAL);
    print_footer();
}
```

## Observed stdout

```text
=== Study Tracker ===
Learner: Rahim
Daily goal: 90 minutes
Keep learning, one session at a time.
```

## Common pitfalls and limits

- Calling a function with no definition does not work. Defining it without calling it produces no output.
- A constant needs a type. Use `i32` here; detailed integer ranges come next level.
- Put placeholder values after the format string. Comments beginning with `//` do not print.
- A compiling empty starter is unfinished. Compare the actual report and requirements.

## Source checks

[Rust Book: functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html). These references describe the language constructs used here. The exact output above comes from running this checkpoint, not from a promise about all future versions of the app.
