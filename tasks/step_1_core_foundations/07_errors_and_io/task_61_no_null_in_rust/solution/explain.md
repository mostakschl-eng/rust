# Solution Explanation: Task 61 (No NULL in Rust)

Let's dissect how `Option<T>` provides compile-time protection against null pointer exceptions!

---

## 🔍 The Solution Code

```rust
struct UserProfile {
    username: String,
    email: String,
    middle_name: Option<String>,
    phone_number: Option<String>,
}

fn main() {
    let user = UserProfile {
        username: String::from("Farhan"),
        email: String::from("farhan@tech.com"),
        middle_name: None,
        phone_number: Some(String::from("+880-1711-000000")),
    };

    match &user.middle_name {
        Some(m) => println!("Middle Name: {}", m),
        None => println!("Middle Name: [Not Provided]"),
    }
}
```

---

## 🧩 Key Takeaways

1. **Compulsory Safety:**
   * In languages with `null`, you can write `user.middle_name.length()` and crash if it's null.
   * In Rust, `user.middle_name` is an `Option<String>`. You **cannot** call `.len()` on an `Option` without unwrapping it first!
2. **Borrowing inside match (`&user.middle_name`):**
   * Using `&user.middle_name` borrows the inner string reference without moving ownership out of the struct.


## Reading the Result Carefully

An optional profile field is a labeled envelope: either a value is inside, or it explicitly says absent.

| Symbol or term | Meaning |
|---|---|
| `Option<T>` | possible absence |
| `Some(value)` | present |
| `None` | absent |
| `match` | handle variants |

Safe references cannot be null, but raw pointers can be null. Option does not prevent panics if you choose unwrap on None.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
