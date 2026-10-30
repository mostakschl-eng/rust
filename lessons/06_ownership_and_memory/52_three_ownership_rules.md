# Lesson 52: The Three Sacred Rules of Rust Ownership

In Lesson 51, we learned that dynamic data like `String` lives on the **Heap**, while its controller lives on the **Stack**.

How does Rust ensure that heap memory is freed at the exact right millisecond—**without a garbage collector and without crashes**?

Everything in Rust boils down to **Three Sacred Rules of Ownership**. Memorize these three rules, and Rust will become your closest ally! 📜

---

## 1. The Three Sacred Rules of Ownership

> ### 👑 Rule 1: Each value in Rust has an **Owner**.
> ### 👑 Rule 2: There can only be **ONE owner at a time**.
> ### 👑 Rule 3: When the owner goes out of scope, the value is **automatically dropped** (freed from memory)!

Let's dissect each rule with simple everyday life examples!

---

## 2. Rule 1: Every Value Has an Owner

In Rust, variables aren't just floating around randomly in the universe.  
Every piece of data is strictly **owned** by a variable binding:

```rust
fn main() {
    // The variable 's' is the official OWNER of the heap-allocated String "Rustacean":
    let s = String::from("Rustacean");

    println!("The owner says: {}", s);
}
```

---

## 3. Rule 2: There Can Only Be ONE Owner at a Time

### The Real-Life Analogy: A Physical Book 📕
Imagine you own a physical paperback book:
* You have it in your hands. You are the sole owner.
* If your friend Karim asks for the book, and you **hand it over to him**:
  * Karim is now the new owner.
  * **You no longer have the book!**
  * You cannot open the pages and read it anymore because it is in Karim's backpack!

In Rust:
If you assign a heap-allocated variable to a new variable, ownership is **MOVED**:

```rust
fn main() {
    let s1 = String::from("Hello");

    // Ownership of the heap memory MOVES from s1 to s2:
    let s2 = s1;

    // println!("{}", s1); // ❌ COMPILER ERROR: borrow of moved value: `s1`
    println!("{}", s2);    // ✅ Valid! s2 is now the proud sole owner!
}
```

Why does Rust do this?  
Because if both `s1` and `s2` thought they owned the exact same memory, when the program ended, both would try to free that memory at the same time—causing a catastrophic **Double Free Bug** that hackers love to exploit! Rust prevents this at compile time.

---

## 4. Rule 3: Automatic Cleanup at Scope Exit (`drop`)

### The Scope Lifespan `{ ... }`:
A scope is the range within a program for which an item is valid, denoted by curly braces `{ }`:

```rust
fn main() {
    println!("Program started.");

    { // ---> Inner scope begins
        let secret = String::from("SuperSecretPassword");
        println!("Inside scope: {}", secret);
        // 'secret' is fully valid and usable here.
    } // <--- Inner scope ends! 

    // 💥 The moment the closing brace } is hit:
    // Rust automatically calls a special function called `drop`.
    // The heap memory used by 'secret' is returned to the OS immediately!

    // println!("{}", secret); 
    // ❌ COMPILER ERROR: cannot find value `secret` in this scope
}
```

### 📋 Expected Terminal Output:
```text
Program started.
Inside scope: SuperSecretPassword
```

No memory leaks, no garbage collector pauses, no manual `free()` calls! Everything is deterministic.

---

## 5. Scope Visualized

```text
{                      // s is not valid here, since it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid (DROPPED!)
```

---

## 6. Quick Recap Checklist
- [x] **Rule 1:** Every value has a designated owner variable.
- [x] **Rule 2:** There is only ONE owner at any given time.
- [x] **Rule 3:** When the owner's scope `{}` ends, Rust automatically deallocates the value with `drop`.
- [x] Passing a heap value to another variable transfers ownership (a **Move**).
- [x] The moved-from variable becomes invalid and unusable.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/06_ownership_and_memory/task_52_three_ownership_rules/TASK.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_52_three_ownership_rules/TASK.md).
* Next lesson: [Lesson 53: Variable Moves vs Primitive Copies](file:///d:/rust-learning/lessons/06_ownership_and_memory/53_move_vs_copy.md).
