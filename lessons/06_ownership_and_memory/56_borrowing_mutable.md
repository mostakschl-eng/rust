# Lesson 56: Mutable Borrowing: Exclusive References (`&mut T`)

In Lesson 55, we learned how to borrow data for **reading** using `&T`.  
What if you want to let a function **modify your data** without transferring ownership?

For example:
* Appending text to an existing string.
* Sorting numbers inside a vector.
* Updating user preferences in a settings struct.

In Rust, you do this using an **Exclusive Mutable Reference: `&mut T`**! ✏️

---

## 1. The Real-Life Analogy: The Official Editing Pen ✒️

Imagine you are writing a joint contract:
* If 5 people want to **read** the contract, they can all sit around the table and read it together with their eyes (**Shared References `&`**).
* But what if someone wants to **edit** the contract with a pen?
* Only **ONE person** can hold the editing pen at any given moment!
* While that person is actively writing or making edits, no one else can distract them or change words underneath their pen.

In Rust:
> A **Mutable Reference (`&mut`)** gives temporary, exclusive write access to data.  
> You can modify the value, but you do not own it!

---

## 2. Creating and Using a Mutable Reference

Notice the three places where `mut` must be written:
1. The original variable must be declared `mut`.
2. The reference must be created with `&mut`.
3. The function parameter must accept `&mut Type`.

```rust
// This function borrows a mutable reference and appends text to it:
fn append_world(text: &mut String) {
    text.push_str(", World! 🌍");
}

fn main() {
    // 1. Variable must be mutable:
    let mut greeting = String::from("Hello");

    // 2. Pass a mutable reference using &mut:
    append_world(&mut greeting);

    println!("Result: {}", greeting);
}
```

### 📋 Expected Terminal Output:
```text
Result: Hello, World! 🌍
```

`greeting` stayed alive in `main`, but its contents were modified cleanly by `append_world`!

---

## 3. Modifying Numbers via Dereferencing (`*`)

When you have a mutable reference to a primitive number, you use the **dereference operator `*`** to reach through the pointer and update the value:

```rust
fn double_value(num: &mut i32) {
    *num *= 2; // Notice the asterisk (*) to mutate the value behind the reference!
}

fn main() {
    let mut score = 50;
    println!("Original score: {}", score);

    double_value(&mut score);

    println!("Doubled score:  {}", score);
}
```

### 📋 Expected Terminal Output:
```text
Original score: 50
Doubled score:  100
```

---

## 4. The Golden Rule of Mutable References: Exclusivity! 🛑

You can only have **ONE mutable reference** to a particular piece of data in a particular scope:

```rust
fn main() {
    let mut s = String::from("Rust");

    let r1 = &mut s;
    let r2 = &mut s; // ❌ COMPILER ERROR: cannot borrow `s` as mutable more than once at a time!

    println!("{}, {}", r1, r2);
}
```

### Why is Rust so strict about this?
To prevent **Data Races**!  
A data race happens when two pointers try to access the exact same memory at the same time, at least one is writing, and there's no synchronization.  
In languages like C, C++, or Go, data races cause bizarre, hard-to-reproduce bugs in production. Rust stops data races at compile time!

---

## 5. Quick Recap Checklist
- [x] Use `&mut variable` to borrow data with write permissions.
- [x] The original variable must be declared with `let mut`.
- [x] Use `*ref` (dereferencing) to update primitive values behind the reference.
- [x] You can only have **ONE mutable reference** to a value at any time.
- [x] Exclusivity guarantees zero data races and zero memory corruption!

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/06_ownership_and_memory/task_56_borrowing_mutable/TASK.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_56_borrowing_mutable/TASK.md).
* Next lesson: [Lesson 57: The Golden Borrowing Rule: Many Readers OR One Writer](file:///d:/rust-learning/lessons/06_ownership_and_memory/57_borrowing_rules.md).
