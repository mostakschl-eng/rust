# Lesson 59: Slices: Borrowing Parts of Arrays and Strings (`&[T]`, `&str`)

Imagine you have a giant collection:
* An entire encyclopedia containing **1,000 pages**.
* An array of **10,000 temperature readings**.

What if you only want to work with **pages 50 to 75**, or examine the **first 3 words of a sentence**?  
You don't want to copy all that data into a new vector!

In Rust, you can borrow any contiguous portion of a collection using **Slices**! 🍞✂️

---

## 1. The Real-Life Analogy: The Stencil Window 🪟

Imagine you are looking at a 100-meter wall painted with words:
* You hold a cardboard stencil with a small rectangular cut-out window.
* You hold the window up to the wall.
* Through the cut-out, you can see only the words: **"RUST ROCKS"**.
* You didn't chip the bricks out of the wall; you didn't paint a new wall.
* You are just holding a **view** into a specific portion of the wall!

In Rust:
> A **Slice** is a reference to a contiguous sequence of elements in a collection rather than the whole collection.  
> Slices are fat pointers: they store a **starting memory address** and a **length**.

---

## 2. String Slices: `&str`

We first met `&str` in Lesson 38. Now let's see how range syntax `[start..end]` extracts slices:

```rust
fn main() {
    let message = String::from("Hello, Beautiful World!");

    // Take slice from index 0 up to (not including) index 5:
    let hello = &message[0..5];

    // Take slice from index 7 up to index 16:
    let beautiful = &message[7..16];

    println!("Slice 1: '{}'", hello);
    println!("Slice 2: '{}'", beautiful);
}
```

### 📋 Expected Terminal Output:
```text
Slice 1: 'Hello'
Slice 2: 'Beautiful'
```

---

## 3. Slice Range Shorthands

| Syntax | What It Slices | Example on `"Hello"` |
|---|---|---|
| `[0..2]` | From index 0 up to index 2 (excludes 2) | `"He"` |
| `[..2]` | From the very start up to index 2 | `"He"` |
| `[3..len]` | From index 3 up to the end | `"lo"` |
| `[3..]` | From index 3 up to the end | `"lo"` |
| `[..]` | Entire string / array as a slice | `"Hello"` |

---

## 4. Array Slices: `&[T]`

Slices don't just work on text; they work on **any collection** (arrays, vectors)!

```rust
fn main() {
    let temperatures = [22, 25, 29, 31, 28, 24, 20];

    // Borrow the middle weekend days (indices 2, 3, 4):
    let weekend_peak: &[i32] = &temperatures[2..5];

    println!("Full week count: {}", temperatures.len());
    println!("Weekend peak:    {:?} (Length: {})", weekend_peak, weekend_peak.len());
    println!("First peak day:  {}°C", weekend_peak[0]);
}
```

### 📋 Expected Terminal Output:
```text
Full week count: 7
Weekend peak:    [29, 31, 28] (Length: 3)
First peak day:  29°C
```

Notice:
* The type of a slice of integers is written **`&[i32]`**.
* You can index into a slice starting at `[0]`! For `weekend_peak`, `[0]` accesses the first element of the slice (`29`).

---

## 5. Writing Flexible Functions Using Slices

Look at this pro-level Rust tip:
```rust
// ❌ Clunky: Only accepts a Vector of i32
fn sum_vector(v: &Vec<i32>) -> i32 { ... }

// ✅ Idiomatic Rust: Accepts Slices (&[i32])!
// Can accept a Vector (&vec[..]), a fixed Array (&array[..]), OR a sub-slice!
fn sum_slice(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

fn main() {
    let my_vec = vec![1, 2, 3];
    let my_arr = [10, 20, 30, 40];

    println!("Vec sum:   {}", sum_slice(&my_vec));
    println!("Array sum: {}", sum_slice(&my_arr));
    println!("Sub-slice: {}", sum_slice(&my_arr[1..3])); // Sum of 20 and 30!
}
```

---

## 6. Quick Recap Checklist
- [x] Slices are references to a contiguous section of data (`&[T]` or `&str`).
- [x] Created using range syntax: `&collection[start..end]`.
- [x] Omit the start to begin at 0 (`&data[..3]`); omit the end to slice until finish (`&data[3..]`).
- [x] Slices do not copy or allocate memory; they only hold a pointer and length.
- [x] Prefer `&[T]` over `&Vec<T>` and `&str` over `&String` in function arguments!

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/06_ownership_and_memory/task_59_slices_deep_dive/TASK.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_59_slices_deep_dive/TASK.md).
* Next lesson: [Lesson 60: Introduction to Lifetimes (`'a` annotations)](file:///d:/rust-learning/lessons/06_ownership_and_memory/60_lifetimes_intro.md).
