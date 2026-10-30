# Lesson 55: Borrowing: Shared References (`&T` - Read Only)

In Lesson 53 and 54, we faced a major dilemma:
* If we pass a `String` into a function, it **moves ownership**, destroying the variable in `main`.
* If we call `.clone()`, it wastes CPU and RAM making expensive duplicates on the heap.

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
// This function BORROWS a String reference (&String) instead of taking ownership:
fn calculate_length(text: &String) -> usize {
    text.len() // We can read its length!
} // 'text' goes out of scope here, but because it is a reference, NOTHING IS DROPPED!

fn main() {
    let message = String::from("Rustacean");

    // Pass a reference using '&':
    let length = calculate_length(&message);

    // 'message' is STILL completely alive and usable!
    println!("The length of '{}' is {} characters.", message, length);
}
```

### 📋 Expected Terminal Output:
```text
The length of 'Rustacean' is 9 characters.
```

Notice:
1. `&message` creates a reference pointing to `message`.
2. `calculate_length` takes `&String`.
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

A reference is just a tiny 8-byte pointer on the stack pointing directly to the original data.  
Borrowing costs virtually **zero performance overhead**!

---

## 4. Shared References Are Immutable by Default 🔒

When you borrow something using `&`, you have **read-only access**.  
You are looking at the library book; you are **not** allowed to take out a marker and scribble over the pages!

```rust
fn try_to_modify(s: &String) {
    // s.push_str(" World"); 
    // ❌ COMPILER ERROR: `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable!
}
```

If you need to change borrowed data, you need an **Exclusive Mutable Reference (`&mut`)**, which we will explore in Lesson 56!

---

## 5. Multiple Readers: Unlimited Shared References! 👥

Because shared references are strictly read-only, there is zero risk of data corruption.  
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

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/06_ownership_and_memory/task_55_borrowing_shared/TASK.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_55_borrowing_shared/TASK.md).
* Next lesson: [Lesson 56: Mutable Borrowing: Exclusive References (`&mut T`)](file:///d:/rust-learning/lessons/06_ownership_and_memory/56_borrowing_mutable.md).
