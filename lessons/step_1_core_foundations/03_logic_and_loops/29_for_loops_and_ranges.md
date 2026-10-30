# Lesson 29: Counting & Ranges (`for` Loops)

Welcome to Lesson 29! 

In Lesson 26 and 28, we looked at `loop` and `while`. But notice how in both of them, we had to remember to write `count += 1;` manually. If you forgot that one line, your program would freeze!

A `for` loop is useful for stepping through an iterator. A finite range removes the need to update a counter manually; its body can still hang, panic, or contain another nonterminating loop.

---

## 1. Rust Ranges: Two Ways to Count

In Rust, you describe a sequence of numbers using **Range Syntax**:

### 1. The Exclusive Range: `1..5` (Stop Before)
```rust
1..5 // Numbers: 1, 2, 3, 4 (Stops BEFORE 5!)
```

### 2. The Inclusive Range: `1..=5` (Includes End with `=`)
```rust
1..=5 // Numbers: 1, 2, 3, 4, 5 (Includes 5!)
```
> **Memory Trick:** If you see the equals sign `=`, it means: *"Include the ending number too!"*

---

## 2. Your First `for` Loop in Rust

Look at how remarkably readable a `for` loop is:

```rust
fn main() {
    println!("Printing numbers 1 to 5:");

    for number in 1..=5 {
        println!("Number: {}", number);
    }

    println!("Done counting!");
}
```

### The Output on Screen:
```text
Printing numbers 1 to 5:
Number: 1
Number: 2
Number: 3
Number: 4
Number: 5
Done counting!
```

Notice:
1. No `mut number` needed!
2. No `number += 1` needed! Rust advances the number automatically!
3. No `break` needed! Rust stops automatically when the range reaches the end!

---

## 3. Counting Backwards: The `.rev()` Method

What if you want to count backwards (e.g. 5, 4, 3, 2, 1)?
You wrap the range in parentheses and call **`.rev()`** (short for *reverse*):

```rust
fn main() {
    println!("Blastoff in:");

    for second in (1..=3).rev() {
        println!("{}...", second);
    }

    println!("🚀 Liftoff!");
}
```

### The Output on Screen:
```text
Blastoff in:
3...
2...
1...
🚀 Liftoff!
```

---

## 4. Generating Multiplication Tables

With a single `for` loop, you can generate complex tables in 4 lines of code:

```rust
fn main() {
    let multiplier = 7;
    println!("=== 7 TIMES TABLE ===");

    for i in 1..=10 {
        println!("{} x {:>2} = {:>2}", multiplier, i, multiplier * i);
    }
}
```

### The Output on Screen:
```text
=== 7 TIMES TABLE ===
7 x  1 =  7
7 x  2 = 14
7 x  3 = 21
7 x  4 = 28
7 x  5 = 35
7 x  6 = 42
7 x  7 = 49
7 x  8 = 56
7 x  9 = 63
7 x 10 = 70
```

---

## 5. Summary Checklist
- [x] `for item in range { ... }` is a useful way to iterate over a range or collection.
- [x] `1..5` counts 1, 2, 3, 4 (excludes 5).
- [x] `1..=5` counts 1, 2, 3, 4, 5 (includes 5).
- [x] `(1..=5).rev()` counts in reverse order.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/03_logic_and_loops/task_29_for_loops_and_ranges/TASK.md`](../../../tasks/step_1_core_foundations/03_logic_and_loops/task_29_for_loops_and_ranges/TASK.md)**.
1. Read the task requirements.
2. Build an automated workout gym rep counter that counts reps 1 to 10 and announces sets.
3. Run `cargo run` and test your for loop!

## Symbol reference

| Symbol | Meaning |
|---|---|
| `for ... in` | Visit each iterator item |
| `..` | Range excluding the end |
| `..=` | Range including the end |
| `.rev()` | Reverse a double-ended iterator |

## Common pitfalls and limits

A descending range such as 5..1 is empty; reverse an ascending range with rev(). A finite range does not protect against a body that hangs or never returns.
