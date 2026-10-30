# Lesson 67: Reading Keyboard Input from Terminal (`std::io::stdin`)

Up until now, all of our program variables were hardcoded: `let age = 25;`.  
Real software interacts with **live human users**!
* Asking for a user's name.
* Reading interactive game moves.
* Accepting numerical commands in terminal tools.

In Rust, keyboard input is handled via the standard I/O library: **`std::io::stdin()`**! ⌨️

---

## 1. The Real-Life Analogy: The Listening Microphone 🎙️

Imagine a stage microphone connected to a recording tape:
* Before the speaker talks, you must provide a **blank tape** ready to record (**A Mutable `String` Buffer**).
* You turn on the microphone (**`io::stdin()`**).
* The speaker speaks into the microphone and presses the "Send" button (**Pressing Enter**).
* The sound waves are captured and written directly onto your blank tape!

In Rust:
> You create an empty, mutable `String` buffer.  
> You tell `io::stdin().read_line(&mut buffer)` to listen to the keyboard and store whatever the user types!

---

## 2. Your First Interactive Input Program

Look at how terminal input works:

```rust
use std::io;

fn main() {
    println!("=== TERMINAL GREETING APP ===");
    println!("Please enter your name:");

    // 1. Create an empty string buffer to hold the user's keystrokes:
    let mut user_input = String::new();

    // 2. Read keyboard input into our mutable buffer:
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read keyboard input!");

    // 3. Trim the newline character ('\n') created when the user pressed Enter:
    let clean_name = user_input.trim();

    println!("Welcome to Rust programming, {}! 🦀", clean_name);
}
```

---

## 3. The Crucial `.trim()` Step ⚠️

When a user types `"Alice"` and hits **Enter** on their keyboard:
* The computer doesn't just read `"Alice"`.
* It reads `"Alice\n"` on Linux/macOS, or `"Alice\r\n"` on Windows!
* If you don't call **`.trim()`**, any future printing or comparison will include an unwanted line break:

```rust
let name = user_input.trim(); // Strips away '\r' and '\n'!
```

---

## 4. Reading and Parsing Numbers from User Input

Because `read_line()` always reads raw text (`String`), if you want a **number**, you must parse it:

```rust
use std::io;

fn main() {
    println!("Enter your age:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    // Trim the whitespace and parse as u32:
    match input.trim().parse::<u32>() {
        Ok(age) => println!("Next year you will be {} years old!", age + 1),
        Err(_) => println!("⚠️ That was not a valid whole number!"),
    }
}
```

---

## 5. Summary of Keyboard Input Steps

| Step | Code | Purpose |
|---|---|---|
| 1. Import I/O | `use std::io;` | Brings input/output into scope |
| 2. Create Buffer | `let mut buf = String::new();` | Empty bucket to catch characters |
| 3. Read Input | `io::stdin().read_line(&mut buf)` | Waits for user to press Enter |
| 4. Strip Newline | `buf.trim()` | Cleans whitespace and line endings |
| 5. Convert Type | `buf.trim().parse::<i32>()` | Converts string to number if needed |

---

## 6. Quick Recap Checklist
- [x] Read input using `std::io::stdin().read_line(&mut buffer)`.
- [x] Must pass a mutable reference to an existing `String` buffer.
- [x] Always call `.trim()` on the result to remove the trailing Enter newline (`\n`).
- [x] Convert numerical input using `.trim().parse::<Type>()`.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/07_errors_and_io/task_67_terminal_user_input/TASK.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_67_terminal_user_input/TASK.md).
* Next lesson: [Lesson 68: Reading Files from Hard Drive (`std::fs::read_to_string`)](file:///d:/rust-learning/lessons/07_errors_and_io/68_reading_files.md).
