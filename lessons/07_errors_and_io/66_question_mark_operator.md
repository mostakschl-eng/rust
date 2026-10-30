# Lesson 66: Propagating Errors: The `?` (Question Mark) Operator

In Lesson 64, we learned how to return `Result<T, E>`.  
When writing real software with 5 or 6 steps (open file, read line, parse number, validate range, save record), writing a full `match` statement for every single step creates messy, deeply nested code:

```rust
// ❌ The Messy Way: Matching every single step manually!
let file = match open_file() {
    Ok(f) => f,
    Err(e) => return Err(e),
};
let line = match read_line(file) {
    Ok(l) => l,
    Err(e) => return Err(e),
};
// ... nested nightmare!
```

Rust has one of the most loved features in modern programming to eliminate this boilerplate completely:  
**The `?` (Question Mark) Operator**! ❓⚡

---

## 1. The Real-Life Analogy: The Express Mail Forwarder 📬

Imagine an office assistant reviewing documents:
* Step 1: Open the envelope.
* Step 2: Translate the French text.
* Step 3: Verify the signature.

If at **any step** something is wrong (e.g. envelope is empty or signature is forged):
* The assistant doesn't start crying or write 50 lines of excuses.
* They simply stamp **"REJECTED: Forward back to the Sender"** and hand the error straight back up to the manager!

In Rust:
> The **`?` operator** does two things in one character:
> 1. If the result is **`Ok(val)`**, it automatically **unwraps the value** and keeps running the code!
> 2. If the result is **`Err(err)`**, it **returns early from the current function**, passing the error up to the caller!

---

## 2. The `?` Operator in Action

Look at how clean this function becomes:

```rust
fn parse_and_multiply(text_a: &str, text_b: &str) -> Result<i32, std::num::ParseIntError> {
    // If text_a.parse fails, return Err immediately! Otherwise, bind unwrap to 'a':
    let a: i32 = text_a.parse()?;

    // If text_b.parse fails, return Err immediately! Otherwise, bind unwrap to 'b':
    let b: i32 = text_b.parse()?;

    // If both succeeded, return the final result wrapped in Ok:
    Ok(a * b)
}

fn main() {
    println!("Success: {:?}", parse_and_multiply("10", "5"));
    println!("Failure: {:?}", parse_and_multiply("10", "bad_number"));
}
```

### 📋 Expected Terminal Output:
```text
Success: Ok(50)
Failure: Err(ParseIntError { kind: InvalidDigit })
```

Look at `let a: i32 = text_a.parse()?;`!  
That single `?` replaced an entire 6-line `match` block!

---

## 3. The One Golden Rule of `?`: Return Type Compatibility! ⚠️

You can only use `?` inside a function whose **return type matches the error being returned**:
* Inside a function returning `Result<T, E>`, `?` can unwrap a `Result`.
* Inside a function returning `Option<T>`, `?` can unwrap an `Option`.
* If you try to use `?` inside a function that returns nothing (`()`), the compiler will halt you:

```rust
fn main() {
    let num: i32 = "42".parse()?; 
    // ❌ COMPILER ERROR: the `?` operator can only be used in a function that returns `Result` or `Option`
}
```

---

## 4. Returning `Result` Directly from `main()`! 🌟

Did you know `main()` in Rust can actually return a `Result`?  
This allows you to use `?` directly inside your `main` function:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let port: u16 = "8080".parse()?;
    println!("Server listening on port: {}", port);

    Ok(()) // Everything succeeded!
}
```

If anything fails, `main` returns the error and terminates with a non-zero exit code automatically.

---

## 5. Quick Recap Checklist
- [x] The `?` operator automatically unwraps `Ok` or early-returns `Err`.
- [x] Eliminates deeply nested `match` error handling boilerplate.
- [x] Can only be used inside functions that return a compatible `Result` or `Option`.
- [x] `fn main() -> Result<(), E>` allows using `?` at the top level.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/07_errors_and_io/task_66_question_mark_operator/TASK.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_66_question_mark_operator/TASK.md).
* Next lesson: [Lesson 67: Reading Keyboard Input from Terminal (`std::io::stdin`)](file:///d:/rust-learning/lessons/07_errors_and_io/67_terminal_user_input.md).
