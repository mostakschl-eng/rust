# Lesson 33: Fixed-Size Lists: Arrays

In Lessons 31 and 32, we learned about **Tuples**, which group items of **different types** (`&str`, `i32`, `f64`).

Now imagine you want to store:
* The high temperatures for the **7 days of the week**.
* The **12 months** of the year.
* The scores of **5 exam tests**.

All of these items have the **exact same type**!  
For this, Rust gives us **Arrays**! 📊

---

## 1. The Real-Life Analogy: The Egg Carton 🥚

Think of a standard egg carton:
* It has a fixed number of slots: exactly **12 slots**.
* Every single slot holds the **exact same kind of item**: an egg!
* You cannot fit an orange or a brick inside the egg slot.
* You cannot suddenly turn a 12-egg carton into a 15-egg carton.

In Rust:
> An **Array** is a fixed-size list where **every single element must have the exact same data type**.  
> An array stores its elements inline, wherever the array itself is stored. A local array is commonly on the stack, but arrays can also be inside heap allocations.

---

## 2. Your First Array in Rust

Arrays are written using **square brackets `[ ]`**, separated by commas:

```rust
fn main() {
    // An array of 5 integers:
    let scores = [95, 88, 72, 99, 84];

    println!("First score:  {}", scores[0]);
    println!("Second score: {}", scores[1]);
    println!("Third score:  {}", scores[2]);

    // Check how many items are in the array:
    println!("Total scores count: {}", scores.len());
}
```

### 📋 Expected Terminal Output:
```text
First score:  95
Second score: 88
Third score:  72
Total scores count: 5
```

---

## 3. The Explicit Array Type Syntax: `[Type; Size]`

In Rust, the type of an array includes **both** the data type AND the number of elements:

```rust
let weekdays: [&str; 5] = ["Mon", "Tue", "Wed", "Thu", "Fri"];
let prime_numbers: [i32; 4] = [2, 3, 5, 7];
```

| Type Notation | What It Means |
|---|---|
| `[i32; 5]` | An array of 5 integers (`i32`) |
| `[f64; 3]` | An array of 3 decimals (`f64`) |
| `[bool; 10]` | An array of 10 booleans |
| `[&str; 4]` | An array of 4 text slices |

> [!IMPORTANT]
> The size of an array is part of its type!  
> An array `[i32; 4]` and an array `[i32; 5]` are considered **two completely different types** by the Rust compiler.

---

## 4. The Repetition Shortcut: `[DefaultValue; Count]`

Imagine you want an array of 50 zeros (e.g. initializing an audio buffer or game grid).  
Writing `0, 0, 0, ...` fifty times would be terrible!

Rust has a handy shortcut:
```rust
let zeroes = [0; 50]; // Creates [0, 0, 0, ... 50 times!]
let active_flags = [true; 5]; // [true, true, true, true, true]
```

```rust
fn main() {
    let buffer = [0; 5];
    println!("Buffer contents: {:?}", buffer);
}
```
**Output:**
```text
Buffer contents: [0, 0, 0, 0, 0]
```

---

## 5. Printing Arrays: Always Use `{:?}`

Just like tuples, an array cannot be printed with `{}`.  
Always use `{:?}` for compact debug printing, or `{:#?}` for pretty multi-line printing:

```rust
fn main() {
    let colors = ["Red", "Green", "Blue"];
    println!("Standard debug: {:?}", colors);
    println!("Pretty-print debug:\n{:#?}", colors);
}
```

---

## 6. Mutable Arrays: Updating Items

With `let mut`, you can replace values inside the array, but **you cannot add or remove items**:

```rust
fn main() {
    let mut temperatures = [72, 75, 68];
    println!("Before: {:?}", temperatures);

    temperatures[1] = 80; // Update 75 to 80
    println!("After:  {:?}", temperatures);
}
```

---

## 7. Common Beginner Traps ⚠️

### Trap 1: Trying to Mix Different Types
All items in an array must be the same type:
```rust
let invalid = [10, "Twenty", 30]; // ❌ COMPILER ERROR: expected integer, found &str
```
*(If you need different types, use a **Tuple** `(10, "Twenty", 30)` instead!)*

### Trap 2: Semicolon vs Comma Inside Brackets
* `[0, 5]` means an array with two elements: `0` and `5`.
* `[0; 5]` means an array with five zeros: `[0, 0, 0, 0, 0]`.

---

## 8. Quick Recap Checklist
- [x] Arrays store multiple items of the **same data type** inside `[ ]`.
- [x] The size of an array is fixed and cannot grow or shrink.
- [x] Array type signature: `[Type; Count]`, e.g. `[i32; 5]`.
- [x] Shortcut to initialize with default values: `[0; 100]`.
- [x] Get the size using `.len()`.
- [x] Print arrays using debug formatting `{:?}`.

---

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `[T; N]` | Array type with element type T and count N |
| `[1; 5]` | Five copies of the integer 1 |
| `[i]` | Read an element by zero-based index |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_33_arrays/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_33_arrays/TASK.md).
* Next lesson: [Lesson 34: Accessing Array Elements & Index Bounds Checking](34_array_indexing.md).
