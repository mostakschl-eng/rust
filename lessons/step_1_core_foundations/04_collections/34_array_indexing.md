# Lesson 34: Accessing Array Elements & Index Bounds Checking

Now that we know how to create arrays, let's learn how to read elements safely and explore how Rust prevents one of the most famous security bugs in computer history: **Buffer Overflow**! 🛡️

---

## 1. The Real-Life Analogy: House Addresses on a Street 🏡

Imagine a street with 5 houses built in a row:
* House 1 is at index **0**
* House 2 is at index **1**
* House 3 is at index **2**
* House 4 is at index **3**
* House 5 is at index **4**

If a mail carrier is asked to deliver a letter to **House 10**, what happens?  
There is no House 10! The street ends at House 5 (index 4).

In programming:
* Trying to access an index that doesn't exist is called an **Out-of-Bounds Error**.
* Unchecked out-of-bounds access in C or C++ is undefined behavior: it may crash, corrupt data, or expose memory.
* **Rust protects you:** Safe Rust checks array indexing and prevents that access from reading or writing beyond the array.

---

## 2. Reading Elements Using Zero-Based Indexing

Always remember: **Counting starts at 0**!

```rust
fn main() {
    let fruits = ["Apple", "Banana", "Cherry", "Date", "Elderberry"];

    // 1. Reading by index:
    println!("Index 0: {}", fruits[0]); // Apple
    println!("Index 2: {}", fruits[2]); // Cherry

    // 2. Finding the last element safely:
    let last_index = fruits.len() - 1; // 5 - 1 = 4
    println!("Last Fruit: {}", fruits[last_index]); // Elderberry
}
```

### 📋 Expected Terminal Output:
```text
Index 0: Apple
Index 2: Cherry
Last Fruit: Elderberry
```

---

## 3. What Happens If You Go Out of Bounds? 🚨

Let's see what Rust does if you try to access an invalid index:

```rust
fn main() {
    let fruits = ["Apple", "Banana", "Cherry"];

    // Index 0: Apple, Index 1: Banana, Index 2: Cherry
    // There is NO index 5!
    let index: usize = "5".parse().expect("valid demo index");
    let mystery = fruits[index]; 
    println!("{}", mystery);
}
```

Here the index is read at runtime using `.parse()` (text-to-number conversion); `.expect()` accepts this known-valid demonstration text. Direct `fruits[5]` on this fixed array can instead be rejected by the compiler. The runtime lookup triggers a **panic**; exact file locations vary:

```text
thread 'main' panicked at src/main.rs:6:19:
index out of bounds: the len is 3 but the index is 5
```

> [!CAUTION]
> A **panic** is Rust's emergency brake.  
> It stops normal execution of the current thread. An unhandled panic in `main` ends this demonstration; other threads and panic handlers are a later topic.

---

## 4. Safe Reading Without Panics: The `.get()` Method!

What if you're writing a real program where a user types in an index, and you want to handle typos gracefully without crashing?

Rust gives you the **`.get(index)`** method!  
Instead of panicking, `.get()` returns an `Option`:
* `Some(&value)` if the index exists.
* `None` if the index was out of bounds!

```rust
fn main() {
    let colors = ["Red", "Green", "Blue"];

    // Safe lookup at index 1:
    match colors.get(1) {
        Some(color) => println!("Found color: {}", color),
        None => println!("No color at this index!"),
    }

    // Safe lookup at out-of-bounds index 10:
    match colors.get(10) {
        Some(color) => println!("Found color: {}", color),
        None => println!("No color at this index! (Safely handled!)"),
    }
}
```

### 📋 Expected Terminal Output:
```text
Found color: Green
No color at this index! (Safely handled!)
```

---

## 5. Summary Table: Indexing Methods

| Method | Syntax | What Happens If Out of Bounds? | Best Used For |
|---|---|---|---|
| Direct Indexing | `arr[i]` | Panics immediately! 💥 | When you are 100% sure the index is valid (e.g. inside a loop bounds). |
| Safe Get | `arr.get(i)` | Returns `None` gracefully 🛡️ | User input or external dynamic data. |

---

## 6. Common Beginner Traps ⚠️

### Trap: Forgetting That the Last Index is `len - 1`
If an array has 5 items (`len() == 5`):
* Valid indices are: `0, 1, 2, 3, 4`.
* Trying to access `arr[5]` will **always panic**!
* For a **nonempty** array, the last item is `arr[arr.len() - 1]`. For possibly empty collections, use `.last()`, which returns `Some(reference)` or `None`.

---

## 7. Quick Recap Checklist
- [x] Indexing starts at `0`.
- [x] Access elements directly with `arr[index]`.
- [x] Array length is obtained with `arr.len()`.
- [x] Out-of-bounds access causes a panic to protect computer memory.
- [x] Use `.get(index)` for crash-free access when bounds are uncertain.

---

## Small prerequisite bridge: optional results and matching

Think of `Option<T>` as a parcel that either contains a value (`Some(value)`) or is empty (`None`). Here `.get(i)` returns `Option<&T>`: it borrows an element instead of copying or moving it. `Some(value)` is a **pattern** that gives the borrowed element the local name `value`; it is not a new lookup. `match` chooses one arm and must handle both possibilities. `=>` separates a pattern from its action. Lessons 46–50 develop enums and matching fully.


## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `[i]` | Index; panics if a runtime index is invalid |
| `.get(i)` | Return an optional borrowed element |
| `Some(code) =>` | Name and handle a found element |
| `None =>` | Handle absence |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_34_array_indexing/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_34_array_indexing/TASK.md).
* Next lesson: [Lesson 35: Dynamic Lists: Vectors](35_vectors.md).
