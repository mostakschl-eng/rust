# Lesson 64: Recoverable Errors: The `Result<T, E>` Enum (`Ok` vs `Err`)

In Lesson 62 and 63, we handled **`Option<T>`**, which answers:
> *"Does the value exist, or is it missing?"*

Now, what about operations that can **fail with a specific reason**?
* Trying to open a file: What if the file was deleted? Or permissions denied?
* Connecting to a website: What if the Wi-Fi is down? Or the server timed out?
* Parsing a number from text: What if someone typed `"abc"` instead of `"123"`?

In other languages, these throw runtime `Exceptions` (try/catch blocks) that can be easily forgotten.  
In Rust, all recoverable errors are returned as a **`Result<T, E>`**! 🚦

---

## 1. How is `Result<T, E>` Defined?

Just like `Option`, `Result` is built directly into Rust's core prelude:

```rust
enum Result<T, E> {
    Ok(T),  // The operation succeeded; contains the successful value T
    Err(E), // The operation failed; contains the error explanation E
}
```

* **`T`**: The type of data returned on success (e.g. `i32`, `String`, `File`).
* **`E`**: The type of error returned on failure (e.g. `String`, `std::io::Error`).

---

## 2. A Real-World Example: Parsing Numbers

Let's look at Rust's built-in `.parse()` method, which converts text into an integer:

```rust
fn main() {
    let good_text = "42";
    let bad_text = "forty-two";

    // .parse::<i32>() returns a Result<i32, ParseIntError>:
    let parsed_good = good_text.parse::<i32>();
    let parsed_bad = bad_text.parse::<i32>();

    println!("Good parse: {:?}", parsed_good);
    println!("Bad parse:  {:?}", parsed_bad);
}
```

### 📋 Expected Terminal Output:
```text
Good parse: Ok(42)
Bad parse:  Err(ParseIntError { kind: InvalidDigit })
```

Look how honest the return type is!  
It didn't crash; it handed you an `Err` explaining that `"forty-two"` contains invalid digits!

---

## 3. Handling `Result` with `match`

The gold standard for handling results is `match`:

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero!"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let attempts = [(100.0, 5.0), (50.0, 0.0)];

    for (num, den) in attempts {
        match divide(num, den) {
            Ok(answer) => println!("{num} / {den} = {:.2}", answer),
            Err(err_msg) => println!("⚠️ Math Error: {}", err_msg),
        }
    }
}
```

### 📋 Expected Terminal Output:
```text
100 / 5 = 20.00
⚠️ Math Error: Cannot divide by zero!
```

---

## 4. Helpful Query Methods on `Result`

Just like `Option`, `Result` provides quick query and unwrapping methods:

| Method | What It Does |
|---|---|
| `res.is_ok()` | Returns `true` if `Ok` |
| `res.is_err()` | Returns `true` if `Err` |
| `res.unwrap_or(default)` | Returns inner value on `Ok`, or returns `default` on `Err` |
| `res.expect("msg")` | Returns inner value on `Ok`, or panics with custom message on `Err` |

```rust
let input = "not a number";
let number = input.parse::<i32>().unwrap_or(0); // Safely defaults to 0!
println!("Number: {}", number); // 0
```

---

## 5. `Option` vs `Result`: Which When?

| Scenario | Use `Option<T>` | Use `Result<T, E>` |
|---|:---:|:---:|
| Missing value is normal and expected (e.g. optional middle name) | ✅ | |
| Value wasn't found in a search (`.find()`) | ✅ | |
| Operation could fail due to external factors (files, network, parsing) | | ✅ |
| The caller needs to know **WHY** it failed | | ✅ |

---

## 6. Quick Recap Checklist
- [x] Recoverable errors in Rust return `Result<T, E>`.
- [x] Two variants: `Ok(T)` for success, `Err(E)` for failure.
- [x] No unhandled exceptions: compiler requires handling both outcomes.
- [x] Use `match` to handle success and failure paths explicitly.
- [x] Use `.unwrap_or(default)` to fall back gracefully on errors.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/07_errors_and_io/task_64_result_enum/TASK.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_64_result_enum/TASK.md).
* Next lesson: [Lesson 65: Unrecoverable Errors: The `panic!` Macro](file:///d:/rust-learning/lessons/07_errors_and_io/65_panic_macro.md).
