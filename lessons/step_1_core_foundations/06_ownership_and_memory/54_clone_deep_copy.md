# Lesson 54: Deep Copying with `.clone()`

In Lesson 53, we learned that assigning a `String` moves ownership because duplicating heap data automatically would be too slow:
```rust
let s1 = String::from("Hello");
let s2 = s1; // s1 moved!
```

What if you **really, truly want two completely independent copies** of the heap data?  
What if you want `s1` to keep its own "Hello" and `s2` to have its own separate "Hello"?

In Rust, you do this explicitly with the **`.clone()` method**! 👯

---

## 1. The Real-Life Analogy: The Printing Press 🖨️

Think of a book manuscript:
* If you give someone your original hand-written manuscript, you no longer have it (**A Move**).
* But what if you take your manuscript to a commercial print shop?
* The print shop spends time, paper, and ink to manufacture a **complete second physical book**!
* Both books have identical words, but they are **two separate physical items** sitting on different tables.
* If you spill coffee on Book 2, Book 1 stays completely clean!

In Rust:
> **`.clone()`** calls the type's `Clone` implementation. For a `String`, this duplicates its text buffer, producing independent text. It is not universally a deep copy: cloning a reference copies the reference, and cloning an `Rc` or `Arc` shares the same allocation.

---

## 2. Using `.clone()` in Action

Look at how `.clone()` keeps both variables alive:

```rust
fn main() {
    let original = String::from("Rustacean");

    // Explicitly make a deep copy:
    let duplicate = original.clone();

    println!("Original:  {}", original);
    println!("Duplicate: {}", duplicate);
}
```

### 📋 Expected Terminal Output:
```text
Original:  Rustacean
Duplicate: Rustacean
```

Both `original` and `duplicate` are valid and point to their own distinct memory addresses on the heap!

---

## 3. Deep Copy Memory Diagram 🧠

```text
THE STACK                                  THE HEAP
+-----------------------+                  +---+---+---+---+
| original ptr: 0x1000  | ---------------> | R | u | s | t | (Address 0x1000)
+-----------------------+                  +---+---+---+---+

+-----------------------+                  +---+---+---+---+
| duplicate ptr: 0x5000 | ---------------> | R | u | s | t | (Address 0x5000)
+-----------------------+                  +---+---+---+---+
```

Because they have different pointers, modifying one will never affect the other:

```rust
fn main() {
    let s1 = String::from("Hello");
    let mut s2 = s1.clone();

    s2.push_str(" World!");

    println!("s1: {}", s1); // Still "Hello"
    println!("s2: {}", s2); // "Hello World!"
}
```

---

## 4. When Should You Use `.clone()`? (And When Should You Avoid It?)

### When to Use `.clone()`:
* When you genuinely need two separate, independent owners modifying their own data.
* When each thread truly needs independent owned data; later, `Arc` can share ownership without duplicating the payload.
* When prototyping and you want to get code compiling quickly before optimizing.

### When to Avoid `.clone()`:
* **Performance warning:** Cloning can allocate and copy substantial data. Its actual cost depends on the type, data size, and workload; measure performance before optimizing.
* If you only need to **read or inspect** data, **do not clone it**! Instead, **Borrow it (`&`)** as explained in the next lesson!

---

## 5. Adding `.clone()` to Custom Structs with `#[derive(Clone)]`

If you create your own struct, you can give it `.clone()` superpowers by simply adding `Clone` to the derive attribute:

```rust
#[derive(Debug, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn main() {
    let cfg1 = ServerConfig {
        host: String::from("localhost"),
        port: 8080,
    };

    let cfg2 = cfg1.clone(); // Clones both the String and the u16!

    println!("Config 1: {:?}", cfg1);
    println!("Config 2: {:?}", cfg2);
}
```

---

## 6. Quick Recap Checklist
- [x] For String, `.clone()` duplicates its text into an independent buffer; Clone in general is type-defined.
- [x] Both String values here remain valid and own independent text buffers.
- [x] Clone cost depends on the type; it may allocate, copy a small value, or share existing data.
- [x] Use `#[derive(Clone)]` on structs to automatically enable cloning.
- [x] If you only need to read data, prefer Borrowing (`&`) over `.clone()`.

---


## Symbols and Practical Boundaries

Photocopying a document makes independent paper. Cloning a shared-library card instead gives another card for the same resource.

| Symbol or term | Meaning |
|---|---|
| `.clone()` | type-defined duplication |
| `#[derive(Clone)]` | clone each field |
| `mut` | permit editing |

**Common pitfall:** String clones own separate text. References and Rc/Arc clones share their referent; Clone is not a universal deep-copy guarantee.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/06_ownership_and_memory/task_54_clone_deep_copy/TASK.md`](../../../tasks/step_1_core_foundations/06_ownership_and_memory/task_54_clone_deep_copy/TASK.md).
* Next lesson: [Lesson 55: Borrowing: Shared References (`&T`)](55_borrowing_shared.md).

Further reading: [Clone documentation](https://doc.rust-lang.org/std/clone/trait.Clone.html).
