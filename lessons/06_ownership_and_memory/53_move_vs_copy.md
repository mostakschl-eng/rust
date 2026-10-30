# Lesson 53: Variable Moves vs Primitive Copies (Move Semantics)

In Lesson 52, we saw that assigning a `String` moves ownership:
```rust
let s1 = String::from("Hello");
let s2 = s1; // s1 is MOVED and dead!
```

Now look at what happens when we do the exact same thing with integers:
```rust
let x = 42;
let y = x;

println!("x = {}, y = {}", x, y); // ✅ Both x and y are alive and kicking!
```
Wait! Why didn't `x` move? Why is `x` still valid?  
Today we uncover the difference between **Move Semantics** and **Copy Semantics**! 🚚 vs 📋

---

## 1. The Real-Life Analogy: The Heavy Luggage vs The Sticky Note

### The Heavy Luggage (Move Types — `String`, `Vec<T>`):
* Imagine you have a 50kg suitcase packed with fragile electronics.
* You cannot photocopy a 50kg suitcase in one millisecond!
* If you give it to your friend, you **physically transfer ownership**. You now have zero suitcases.

### The Sticky Note (Copy Types — `i32`, `f64`, `bool`):
* Imagine you write the number `42` on a tiny yellow sticky note.
* Your friend says: *"Can I have that number?"*
* You don't have to give up your note!
* You just write `42` on a second sticky note and hand it to them.
* **Both of you now have your own independent copies!** Making a copy took half a microsecond.

---

## 2. The `Copy` Trait

In Rust, types that have a **known, fixed size** and reside entirely on the fast **Stack** implement a special marker called the **`Copy` trait**:

```rust
fn main() {
    let a: i32 = 100;
    let b = a; // Copied byte-for-byte on the stack!

    println!("a: {}, b: {}", a, b); // Perfectly valid!
}
```

### Which Types Implement `Copy`?
All primitive stack types implement `Copy`:
* All integers (`i8`, `i16`, `i32`, `i64`, `u8`, `u32`, `usize`, etc.)
* All floating-point numbers (`f32`, `f64`)
* Booleans (`bool`)
* Characters (`char`)
* Tuples containing **only** `Copy` types (e.g. `(i32, f64)` is Copy, but `(i32, String)` is **NOT** Copy!)
* Fixed arrays containing `Copy` types (e.g. `[i32; 5]`)

---

## 3. Which Types Implement `Move`?
Any type that manages external resources or dynamically allocated **Heap memory** implements `Move` instead of `Copy`:
* `String`
* `Vec<T>`
* `HashMap<K, V>`
* Custom `struct` and `enum` types (by default)

---

## 4. Moves in Function Calls: Giving Away Ownership

Ownership transfer doesn't just happen with `let y = x`.  
It happens whenever you **pass a variable into a function as an argument**!

Look at this code carefully:

```rust
fn take_ownership(text: String) {
    println!("Function received: {}", text);
} // text goes out of scope and is DROPPED here!

fn make_copy(number: i32) {
    println!("Function received number: {}", number);
} // number goes out of scope (stack pops)

fn main() {
    let my_string = String::from("Rust Rules!");
    let my_number = 500;

    make_copy(my_number);
    println!("Number is still alive: {}", my_number); // ✅ Works! (Primitive Copy)

    take_ownership(my_string);
    // println!("String: {}", my_string); 
    // ❌ COMPILER ERROR: borrow of moved value: `my_string`!
}
```

When you called `take_ownership(my_string)`, `my_string` handed its keys over to the function's parameter `text`.  
When the function ended, `text` was dropped, and the heap memory was wiped clean. `my_string` back in `main` is now empty and illegal to use!

---

## 5. Returning Ownership Back

If a function really needs to take ownership, it can return it back to the caller:

```rust
fn add_exclamation(mut s: String) -> String {
    s.push('!');
    s // Return ownership back!
}

fn main() {
    let original = String::from("Hello");
    let updated = add_exclamation(original); // Transfer to function, then return to updated!

    println!("Updated: {}", updated);
}
```

*(While this works, passing ownership back and forth can get exhausting! In Lesson 55, we will discover **Borrowing (`&`)**, which lets functions read data without taking ownership!)*

---

## 6. Quick Recap Checklist
- [x] Primitive stack types (`i32`, `f64`, `bool`, `char`) implement `Copy`.
- [x] Heap-allocated types (`String`, `Vec<T>`) implement `Move`.
- [x] When a value moves, the original variable becomes invalid immediately.
- [x] Passing a variable to a function moves heap data or copies stack data.
- [x] Functions can return ownership back via return values.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/06_ownership_and_memory/task_53_move_vs_copy/TASK.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_53_move_vs_copy/TASK.md).
* Next lesson: [Lesson 54: Deep Copying with `.clone()`](file:///d:/rust-learning/lessons/06_ownership_and_memory/54_clone_deep_copy.md).
