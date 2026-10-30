# Lesson 71: Generics: Writing Code for Any Type (`fn func<T>`)

Welcome to the grand finale of your Rust journey: **Level 8: Advanced Mastery & Concurrency**! 🏆⚡

Imagine you want to write a function that finds the largest item in a list:
* You write one for integers: `fn largest_i32(list: &[i32]) -> i32`
* Then you need one for floating-point numbers: `fn largest_f64(list: &[f64]) -> f64`
* Then you need one for characters: `fn largest_char(list: &[char]) -> char`

All three functions have the **exact same logic**, but you had to write the code three times just because the types were different!  
In software engineering, repeating yourself violates the DRY principle (**Don't Repeat Yourself**).

In Rust, we solve this with **Generics**! 🧬✨

---

## 1. The Real-Life Analogy: The Universal Cookie Cutter 🍪

Think of a baker's cookie cutter:
* The cutter has a specific shape: a star ⭐.
* You can stamp it into **chocolate dough**, **sugar cookie dough**, or **gingerbread dough**.
* The cutter doesn't care what flavor of dough you use—it applies the **same shape and logic to everything**!

In Rust:
> **Generics** are placeholders for types.  
> Instead of writing code for `i32` or `String`, you write code for a generic placeholder called **`T`** (short for "Type")!

---

## 2. Your First Generic Function in Rust

Look at how a generic function is declared using angle brackets `<T>`:

```rust
// 'T' stands for any type!
// We add 'std::fmt::Debug' so we can print it with {:?}:
fn inspect_package<T: std::fmt::Debug>(label: &str, item: T) {
    println!("[PACKAGE: {}] Contents: {:?}", label, item);
}

fn main() {
    // 1. Pass an integer (T = i32):
    inspect_package("Sensors", 42);

    // 2. Pass a floating-point number (T = f64):
    inspect_package("Temperature", 98.6);

    // 3. Pass a string slice (T = &str):
    inspect_package("Secret Code", "DELTA-9");

    // 4. Pass a boolean (T = bool):
    inspect_package("System Status", true);
}
```

### 📋 Expected Terminal Output:
```text
[PACKAGE: Sensors] Contents: 42
[PACKAGE: Temperature] Contents: 98.6
[PACKAGE: Secret Code] Contents: "DELTA-9"
[PACKAGE: System Status] Contents: true
```

One single function handles integers, floats, strings, and booleans with **zero code duplication**!

---

## 3. How to Read Generic Syntax

| Syntax | Pronounced | Meaning |
|---|---|---|
| `<T>` | "Generic over T" | Declares `T` as a generic type placeholder |
| `item: T` | "item of type T" | The parameter accepts whatever type `T` turns out to be |
| `-> T` | "returns T" | The function returns a value of that exact same type |
| `<T, U>` | "Generic over T and U" | Two different generic types (e.g. for pairs of items) |

---

## 4. Multiple Generic Parameters: `<T, U>`

What if a function takes two parameters of different types? Use two letters:

```rust
fn print_pair<T: std::fmt::Display, U: std::fmt::Display>(first: T, second: U) {
    println!("Pair: ({}, {})", first, second);
}

fn main() {
    // T is &str, U is i32:
    print_pair("Ranking", 1);

    // T is bool, U is f64:
    print_pair(true, 99.99);
}
```

---

## 5. Zero-Cost Abstraction: Monomorphization 🚀

Beginners often worry: *"Does using generics make my program slower at runtime?"*

Generics normally use static dispatch: no runtime search is needed to choose which concrete implementation to call. The function body still has its normal costs.  
Rust uses a process called **Monomorphization**:
1. When you compile your code, `rustc` inspects all the concrete types you used (`i32`, `f64`, `&str`).
2. It specializes generic code for concrete types as needed; optimization can inline, merge, or remove code.
3. This can perform like hand-written specialized code. More specializations can increase compile time and binary size; benchmark performance-sensitive code.

---

## 6. Quick Recap Checklist
- [x] Generics let you write flexible functions for any type using placeholders like `<T>`.
- [x] Eliminate repetitive boilerplate across multiple types.
- [x] Conventionally named with single capital letters (`T`, `U`, `V`).
- [x] Monomorphization generates zero-overhead, highly optimized machine code at compile time.

---


## Symbols and Practical Boundaries

One scanner procedure can inspect different parcel types, provided each type supplies the required readable label.

| Symbol or term | Meaning |
|---|---|
| `<T>` | generic type parameter |
| `T: Debug` | required formatting capability |
| `{:?}` | Debug output |

**Common pitfall:** A generic body may use only capabilities established by its bounds. Any T does not automatically support printing or arithmetic. Passing an owned non-Copy item by value still moves it.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/08_advanced_and_concurrency/task_71_generics_functions/TASK.md`](../../../tasks/step_1_core_foundations/08_advanced_and_concurrency/task_71_generics_functions/TASK.md).
* Next lesson: [Lesson 72: Generic Structs & Implementation Blocks](72_generic_structs.md).
