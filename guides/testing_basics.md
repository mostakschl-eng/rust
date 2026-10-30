# Checking your work early

Checking is like measuring a shelf before hanging it. The ruler compares an actual length with the length you wanted. An assertion compares a program's answer with a known answer. This analogy stops at measurement: a check covers only the inputs you gave it. Passing checks do not prove that every input works.

## Part 1: After Lesson 20

You already know functions, parameters, return values, variables, arithmetic and `bool`. Here you learn two macros. Like `println!`, their names end in `!`. You do not need choices, loops or modules yet.

| Symbol | Meaning here |
|---|---|
| `assert_eq!(actual, expected)` | Check that two values are equal |
| `assert!(value)` | Check that a Boolean value is `true` |
| `!` after a macro name | Macro call syntax; not the same as negating a Boolean |
| `-> u32` | The function returns a nonnegative whole number |
| `;` | Finish a statement; omit it on the function's final returned expression |

Try this complete program in `src/main.rs` of a separate practice Cargo package. Keep existing task answers. You can create an empty folder named `checking_practice` beside your existing task folders, then add the following `Cargo.toml` and a `src` folder. This is not another required milestone.

```toml
[package]
name = "checking_practice"
version = "0.1.0"
edition = "2021"
```

```rust
fn study_minutes(sessions: u32, minutes_each: u32) -> u32 {
    sessions * minutes_each
}

fn main() {
    let total = study_minutes(3, 25);
    let ready_to_check = true;
    assert_eq!(total, 75);
    assert_eq!(study_minutes(0, 25), 0);
    assert_eq!(study_minutes(1, 25), 25);
    assert!(ready_to_check);
    println!("Checks passed. Study minutes: {}", total);
}
```

From that practice package's folder, run `cargo check`, then `cargo run`. Program stdout:

```text
Checks passed. Study minutes: 75
```

Passing assertions print nothing. The final line comes from `println!`. `cargo check` checks the code without running these checks. `cargo run` executes them. A failed assertion panics: normal execution stops at that check, and a diagnostic normally appears on stderr. Diagnostic wording and paths vary. These assertions remain active in release builds too. See the official [`assert!`](https://doc.rust-lang.org/std/macro.assert.html) and [`assert_eq!`](https://doc.rust-lang.org/std/macro.assert_eq.html) documentation.

The zero-session and one-session checks cover useful boundaries. This small function is for values whose product fits in `u32`; it does not validate large inputs. In later projects, check empty data and invalid input when those ideas have been taught.

Common pitfalls:

- A successful compile does not establish the result. Run the checks too.
- Write the expected answer independently. Repeating the same calculation on both sides may repeat the same mistake.
- `assert!(true)` checks no business rule. Our Boolean variable only demonstrates syntax. Prefer a meaningful condition once comparisons are taught in Lesson 23.
- Assertions help find programming mistakes. They are not friendly input validation for a person typing an invalid command.

## Part 2: After Lesson 70 (modules)

Now you know modules and `use`. A unit test is a small function that checks one part of your program. Think of giving each shelf its own measurement card. The cards still cover only the measurements you wrote down.

Replace only your separate practice package's `src/main.rs` with this complete program. It keeps the runtime checks and adds a test module:

```rust
fn study_minutes(sessions: u32, minutes_each: u32) -> u32 {
    sessions * minutes_each
}

fn main() {
    assert_eq!(study_minutes(3, 25), 75);
    assert_eq!(study_minutes(0, 25), 0);
    assert_eq!(study_minutes(1, 25), 25);
    println!("Runtime checks passed.");
}

#[cfg(test)]
mod tests {
    use super::study_minutes;

    #[test]
    fn three_sessions() {
        assert_eq!(study_minutes(3, 25), 75);
    }

    #[test]
    fn zero_and_one_session() {
        assert_eq!(study_minutes(0, 25), 0);
        assert_eq!(study_minutes(1, 25), 25);
    }
}
```

| New syntax | Plain meaning |
|---|---|
| `#[...]` | An attribute: an instruction attached to the following item |
| `#[cfg(test)]` | Include this module when building with the test configuration |
| `mod tests { ... }` | A module named `tests`; the dots in this table stand for the code above |
| `super::study_minutes` | The function named `study_minutes` in the parent module |
| `use super::study_minutes;` | Bring that function name into this module's scope |
| `#[test]` | Mark the next function as a test for Rust's test runner |

From the practice package's folder, `cargo run` prints this program stdout:

```text
Runtime checks passed.
```

This runs the assertions in `main`; it does not run the `#[test]` functions. `cargo check --tests` checks the test code without executing it. As the student, you can run `cargo test` from this same folder to execute the marked functions. Expect two passing tests; the runner's full wording, paths and ordering can vary. See the Rust Book on [writing tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) and [test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html).

Common pitfalls:

- `#[test]` belongs above a function, not above an assertion.
- Ordinary `cargo run` excludes this test module. Keep clear which checks you actually ran.
- A check can be wrong too. Compare the requirement, the code, and the expected answer.

Agent verification for this guide used `cargo check`, `cargo check --tests`, and `cargo run` on separate scratch copies. The runtime assertions exercised the same numeric cases as the test functions. The formal test runner was not run because [the repository's agent rules](../AGENTS.md) restrict verification commands.
