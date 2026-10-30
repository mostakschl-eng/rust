# Lesson 17: Scope & Code Blocks (Where Variables Live and Die)

Welcome to Lesson 17! 

Have you ever wondered:
* *How long does a variable live in computer memory?*
* *Can two variables have the same name in different parts of a program?*

The answer lies in one of the most fundamental concepts in computer science: **Scope**!

---

## 1. The Real-Life Analogy: The Hotel Conference Room

Imagine you are attending a workshop at a hotel conference room:
* You walk inside room **101** and close the door (`{`).
* You write some secret notes on a portable whiteboard: `let secret_idea = 42;`.
* While you are inside room 101, you can read the whiteboard.
* When the workshop finishes, you pack up and exit the room (`}`). The room is cleaned and the whiteboard is erased!
* Someone standing out in the hotel lobby cannot see or read your secret whiteboard notes. They vanished when the door closed!

> In Rust:  
> Curly braces `{` and `}` create a **Scope** (a room).  
> Variables created inside a room **die immediately** when the closing `}` is reached!

---

## 2. Outer Scope vs Inner Scope

Here is the golden rule of scopes:
1. **Looking Inward:** Outer code **cannot** look inside an inner room.
2. **Looking Outward:** Inner rooms **can** see variables from the outside hallway!

Look at this code:

```rust
fn main() {
    let global_announcement = "The hotel lobby is open.";

    // Here starts an INNER block room!
    {
        let secret_code = 999;
        println!("Inside room: {}", secret_code);
        println!("Inside room looking out: {}", global_announcement); // Can see outside!
    } // <-- secret_code DIES HERE!

    println!("Back in main: {}", global_announcement);
    // println!("{}", secret_code); // ❌ ERROR! secret_code no longer exists!
}
```

### The Output on Screen:
```text
Inside room: 999
Inside room looking out: The hotel lobby is open.
Back in main: The hotel lobby is open.
```

---

## 3. Temporary Shadowing Inside a Block

You can declare a variable inside an inner scope with the same name as an outer variable. Inside the room, the inner variable takes over. When the room closes, the original outer variable returns untouched!

```rust
fn main() {
    let score = 10;
    println!("Outer score before block: {}", score); // Prints 10

    {
        let score = 500; // Temporary inner score!
        println!("Inner bonus score: {}", score); // Prints 500
    } // Inner score disappears here!

    println!("Outer score after block: {}", score); // Back to 10!
}
```

### The Output on Screen:
```text
Outer score before block: 10
Inner bonus score: 500
Outer score after block: 10
```

---

## 4. Why Does Rust Care So Much About Scope?

In other languages (like C or C++), if a programmer forgets to delete variables when a room closes, the computer's RAM fills up with junk until the computer freezes. This is called a **Memory Leak**.

In Rust:
> The moment a variable reaches its closing `}`, Rust **instantly and automatically frees that memory back to the operating system**!  
> No garbage collector. No memory leaks. Fast and clean!

---

## 5. Summary Checklist
- [x] A scope is created by curly braces `{ ... }`.
- [x] Variables live only inside the scope where they were born.
- [x] Inner scopes can access outer variables, but outer scopes cannot access inner variables.
- [x] Rust automatically cleans up memory the exact moment a scope ends.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_17_scope_blocks/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_17_scope_blocks/TASK.md)**.
1. Read the task requirements.
2. Experiment with inner and outer scopes, temporary bonus multipliers, and variable lifetimes.
3. Run `cargo run` and watch scopes in action!
