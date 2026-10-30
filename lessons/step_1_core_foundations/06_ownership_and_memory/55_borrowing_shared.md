# Lesson 55: Borrowing: Shared References (`&T` - Read Only)

In Lesson 53 and 54, we faced a major dilemma:
* If we pass a `String` into a function, it **moves ownership**, making that binding unavailable in `main`.
* If we call `.clone()`, it duplicates its text, using additional memory and work.

What if you just want to let a function **look at your data without taking ownership and without copying anything**?

In Rust, this is called **Borrowing**, and we do it using **References (`&`)**! 🤝📖

---

## 1. The Real-Life Analogy: The Public Library Book 📚

Think about borrowing a book from the city library:
* You walk into the library.
* You check out a book: you take it home, open the pages, and **read it**.
* **You do NOT own the book.** The library remains the true owner.
* When you are done reading, you return the book back to the shelf.
* The library didn't have to print a brand-new book for you, and the library didn't lose ownership of its book!

In Rust:
> Creating a reference is called **Borrowing**.  
> The ampersand symbol **`&`** creates a reference.  
> You can inspect the value, but when your reference goes out of scope, the underlying data is **NOT dropped** because you don't own it!

---

## 2. Borrowing in Action

Look at how elegant this makes function calls:

```rust
// This function borrows text as &str instead of taking ownership:
fn calculate_length(text: &str) -> usize {
    text.len() // We can read its length!
} // 'text' goes out of scope here, but because it is a reference, NOTHING IS DROPPED!

fn main() {
    let message = String::from("Rustacean");

    // Pass a reference using '&':
    let length = calculate_length(&message);

    // 'message' is STILL completely alive and usable!
    println!("The length of '{}' is {} bytes.", message, length);
}
```

### 📋 Expected Terminal Output:
```text
The length of 'Rustacean' is 9 bytes.
```

Notice:
1. `&message` creates a reference pointing to `message`.
2. `calculate_length` takes `&str`; Rust converts the borrowed `String` to a string slice.
3. `message` was never moved, cloned, or destroyed!

---

## 3. How References Look in Memory 🧠

```text
THE STACK
+--------------------+----------------+
| Reference (&s)     | Pointer: 0x200 | ----+
+--------------------+----------------+     | (Points to the controller on the stack!)
                                            v
+--------------------+----------------+     +-------------------+
| Original 'message' | Pointer: 0x900 | --> | "Rustacean" (HEAP)|
| len: 9             |                |     +-------------------+
| cap: 9             |                |
+--------------------+----------------+
```

A reference to a sized type is pointer-sized (commonly 8 bytes on 64-bit targets). A string slice `&str` also carries a length. References need not be stored on the stack. Borrowing avoids duplicating the underlying text.

---

## 4. Shared References Are Immutable by Default 🔒

When you borrow something using `&`, you have **read-only access**.  
You are looking at the library book; you are **not** allowed to take out a marker and scribble over the pages!

```rust
fn try_to_modify(s: &String) {
    // s.push_str(" World"); 
    // ❌ COMPILER ERROR: `s` is a `&` reference, so the String cannot be borrowed as mutable through this shared reference!
}
```

If you need to change borrowed data, you need an **Exclusive Mutable Reference (`&mut`)**, which we will explore in Lesson 56!

---

## 5. Multiple Readers: Unlimited Shared References! 👥

For ordinary `String` data, a shared reference does not permit mutation. Types with interior mutability, such as `Cell` or `Mutex`, provide controlled exceptions.  
Therefore, Rust lets you have **as many shared references as you want** at the same time:

```rust
fn main() {
    let book = String::from("The Rust Book");

    let reader1 = &book;
    let reader2 = &book;
    let reader3 = &book;

    println!("Reader 1 sees: {}", reader1);
    println!("Reader 2 sees: {}", reader2);
    println!("Reader 3 sees: {}", reader3);
}
```

Ten people can read the same billboard at the same time without any conflict!

---

## 6. Quick Recap Checklist
- [x] Borrowing allows inspecting data without taking ownership.
- [x] Use `&` to create a reference (`&variable`).
- [x] Reference types have an ampersand: `&String`, `&i32`.
- [x] When a reference goes out of scope, the data is NOT dropped.
- [x] Standard references `&T` are immutable (read-only).
- [x] You can have unlimited shared references simultaneously.

---


## Symbols and Practical Boundaries

Several library visitors can read one displayed page without taking ownership of the library book.

| Symbol or term | Meaning |
|---|---|
| `&` | shared borrow |
| `&str` | borrowed UTF-8 text |
| `usize` | length/count type |
| `(a, b)` | tuple |

**Common pitfall:** String length counts bytes, not letters. The manifesto contains 64 ASCII bytes and 9 whitespace-separated words.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/06_ownership_and_memory/task_55_borrowing_shared/TASK.md`](../../../tasks/step_1_core_foundations/06_ownership_and_memory/task_55_borrowing_shared/TASK.md).
* Next lesson: [Lesson 56: Mutable Borrowing: Exclusive References (`&mut T`)](56_borrowing_mutable.md).
