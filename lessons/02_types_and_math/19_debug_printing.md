# Lesson 19: Debug Printing (`{:?}` and `{:#?}`)

Welcome to Lesson 19! 

In earlier lessons, we used `{}` to print numbers and text.
But what happens when you build complex data structures (like arrays, lists, or custom game characters) that don't have a simple text representation?

If you try to print them with `{}`, Rust will say:
> *"I don't know how a human wants this displayed!"*

For developers, Rust has a secret diagnostic lens: **Debug Printing (`{:?}`)**!

---

## 1. User Display `{}` vs Programmer Debug `{:?}`

* **`{}` (Display):** Made for the **end-user** (clean, friendly, no quotes, no brackets).
* **`{:?}` (Debug):** Made for the **developer / programmer** (reveals raw structure, quotes around strings, brackets, and internal values).

Look at this simple example:

```rust
fn main() {
    let username = "Rahim";
    let numbers = (10, 20, 30); // A tuple (group of 3 numbers)

    // With regular Display:
    println!("Hello, {}!", username); // Prints: Hello, Rahim!

    // With Debug {:?}:
    println!("Debug string: {:?}", username); // Prints: "Rahim" (WITH quotes!)
    println!("Debug tuple:  {:?}", numbers);  // Prints: (10, 20, 30)
}
```

### The Output on Screen:
```text
Hello, Rahim!
Debug string: "Rahim"
Debug tuple:  (10, 20, 30)
```

Notice: In debug mode `{:?}`, Rust puts quotes around `"Rahim"` so you can immediately see where text begins and ends!

---

## 2. Pretty Debug Printing: `{:#?}`

What if your data is big and has 10 different fields?
Printing everything on a single long line is messy and hard to read.

If you add a hashtag: **`{:#?}`**, Rust turns on **Pretty-Print Debug Mode**!
It automatically expands the data onto multiple lines with clean indentation:

```rust
fn main() {
    let coordinates = (12.5, 45.8, 99.1);

    println!("Standard Debug: {:?}", coordinates);
    println!("Pretty Debug:\n{:#?}", coordinates);
}
```

### The Output on Screen:
```text
Standard Debug: (12.5, 45.8, 99.1)
Pretty Debug:
(
    12.5,
    45.8,
    99.1,
)
```

---

## 3. The `dbg!` Macro (Fast Inline Debugging)

Rust has an even faster trick when you are debugging: the **`dbg!`** macro!

Instead of writing a full `println!("{:?}", x);`, you can simply write `dbg!(x);`.
Rust will print:
1. The exact file name!
2. The exact line number!
3. The variable name and its value!

```rust
fn main() {
    let score = 100 * 2;
    dbg!(score); // Fast inspection!
}
```

### Output:
```text
[src/main.rs:3:5] score = 200
```

---

## 4. Summary Checklist
- [x] `{}` is for pretty user-facing text.
- [x] `{:?}` is for developer debug inspection.
- [x] `{:#?}` pretty-prints complex data with multi-line indentation.
- [x] `dbg!(...)` prints the line number and variable value for instant troubleshooting.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_19_debug_printing/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_19_debug_printing/TASK.md)**.
1. Read the task requirements.
2. Inspect complex tuples and values using `{:?}`, `{:#?}`, and `dbg!`.
3. Run `cargo run` and see developer diagnostics in action!
