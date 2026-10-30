# Lesson 62: The `Option<T>` Enum (`Some` vs `None`)

In Lesson 61, we learned that Rust replaces `null` with the **`Option<T>`** enum.  
Today, let's explore how `Option<T>` is defined in the standard library and how to use it in your daily code! 📦✨

---

## 1. How is `Option<T>` Defined?

The `Option<T>` enum is so fundamental that it is built directly into Rust's core library and included in the standard **prelude** (meaning you never need to import it!).

Under the hood, it is defined simply as:

```rust
enum Option<T> {
    Some(T), // Holds the value of type T
    None,    // Represents the absence of a value
}
```

* **`<T>`** is a generic placeholder (which we will study in Level 8). It means `Option` can wrap **any type in existence**!
  * `Option<i32>`: An integer that might exist (`Some(42)`) or might not (`None`).
  * `Option<String>`: A string that might exist (`Some("Alice")`) or might not (`None`).
  * `Option<bool>`: A boolean that might exist or might not.

---

## 2. Creating and Inspecting Options

Because `Option`, `Some`, and `None` are in the prelude, you can use them directly without writing `Option::Some`:

```rust
fn main() {
    let has_number: Option<i32> = Some(100);
    let no_number: Option<i32> = None;

    println!("Has number: {:?}", has_number);
    println!("No number:  {:?}", no_number);
}
```

### 📋 Expected Terminal Output:
```text
Has number: Some(100)
No number:  None
```

---

## 3. Real-World Example: Finding an Item in a Vector

Think about looking for an item in a list: what if the item isn't there?

```rust
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // Found an even number! Return immediately!
        }
    }
    None // Looked through the entire slice and found nothing!
}

fn main() {
    let list1 = [1, 3, 5, 8, 9];
    let list2 = [1, 3, 5, 7];

    let result1 = find_even_number(&list1);
    let result2 = find_even_number(&list2);

    match result1 {
        Some(val) => println!("List 1 first even number: {}", val),
        None => println!("List 1 has no even numbers!"),
    }

    match result2 {
        Some(val) => println!("List 2 first even number: {}", val),
        None => println!("List 2 has no even numbers!"),
    }
}
```

### 📋 Expected Terminal Output:
```text
List 1 first even number: 8
List 2 has no even numbers!
```

---

## 4. Helpful Built-in Option Query Methods

Rust provides quick helper methods to inspect an Option without matching:

```rust
fn main() {
    let score: Option<u32> = Some(95);

    // .is_some(): returns true if holding Some
    println!("Has score? {}", score.is_some()); // true

    // .is_none(): returns true if holding None
    println!("Is missing? {}", score.is_none()); // false
}
```

---

## 5. You Cannot Add `Option<i32>` to `i32`! 🛡️

Look at this crucial safety protection:

```rust
let x: i32 = 5;
let y: Option<i32> = Some(5);

// let sum = x + y; 
// ❌ COMPILER ERROR: cannot add `Option<i32>` to `i32`!
```

Why? Because `y` might be `None`!  
Rust **forces you to convert or unwrap the `Option<i32>` into an `i32`** before you can do math with it.  
This makes accidental null arithmetic bugs physically impossible!

In Lesson 63, we will learn all the safe and graceful ways to unwrap options!

---

## 6. Quick Recap Checklist
- [x] `Option<T>` has two variants: `Some(T)` and `None`.
- [x] Automatically loaded in the prelude (no `use` import needed).
- [x] Functions returning optional values return `Option<T>`.
- [x] Check presence using `.is_some()` and `.is_none()`.
- [x] Rust prevents combining `Option<T>` directly with `T` without explicit unwrapping.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/07_errors_and_io/task_62_option_enum/TASK.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_62_option_enum/TASK.md).
* Next lesson: [Lesson 63: Safe Option Unwrapping (`unwrap_or`, `expect`)](file:///d:/rust-learning/lessons/07_errors_and_io/63_safe_option_unwrapping.md).
