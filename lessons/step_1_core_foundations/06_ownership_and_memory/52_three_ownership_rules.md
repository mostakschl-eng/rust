# Lesson 52: The Three Sacred Rules of Rust Ownership

A `String` owns a growable heap buffer through a fixed-size handle. A local handle often lives on the stack, but may also be stored elsewhere.

How does Rust decide who is responsible for cleanup without a tracing garbage collector?

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
An owner can be a variable, a struct field, or another container. Start with a variable binding:

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
Assigning a value whose type does not implement `Copy` transfers ownership. Here the `String` moves:

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
    // The buffer owned by secret is released to the allocator; it need not return to the OS immediately.

    // println!("{}", secret); 
    // ❌ COMPILER ERROR: cannot find value `secret` in this scope
}
```

### 📋 Expected Terminal Output:
```text
Program started.
Inside scope: SuperSecretPassword
```

For this example, cleanup happens at a predictable scope boundary. Safe Rust still allows deliberate leaks, and destructors do not run on process abort or forced termination. Dropping text does not securely erase its old bytes.

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
- [x] **Rule 1:** Every value has an owner; owners include variables, fields, and containers.
- [x] **Rule 2:** There is only ONE owner at any given time.
- [x] **Rule 3:** An owned value is normally dropped when its owner leaves scope; cleanup depends on its type.
- [x] Assigning or passing a non-`Copy` value by value transfers ownership (a **Move**).
- [x] The moved-from variable becomes invalid and unusable.

---


## Symbols and Practical Boundaries

A relay trophy has one responsible holder at a time. A loan lets someone inspect it without becoming the holder.

| Symbol or term | Meaning |
|---|---|
| `{ ... }` | block scope |
| `let next = original` | move for non-Copy values |
| `drop(value)` | consume and drop early |

**Common pitfall:** A moved value is not an empty value. Drop releases a String buffer to its allocator, not necessarily to the OS, and does not erase secrets.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/06_ownership_and_memory/task_52_three_ownership_rules/TASK.md`](../../../tasks/step_1_core_foundations/06_ownership_and_memory/task_52_three_ownership_rules/TASK.md).
* Next lesson: [Lesson 53: Variable Moves vs Primitive Copies](53_move_vs_copy.md).
