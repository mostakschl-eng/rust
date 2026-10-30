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

The **`Copy` trait** permits implicit duplication when a value is assigned or passed by value. Storage location does not decide this: a stack-only struct is not automatically `Copy`, and a number inside a `Box` is still an `i32`.

```rust
fn main() {
    let a: i32 = 100;
    let b = a; // Copies the integer value; a remains usable.

    println!("a: {}, b: {}", a, b); // Perfectly valid!
}
```

Expected output:

```text
a: 100, b: 100
```

### Which Types Implement `Copy`?
Common `Copy` types include:
* All integers (`i8`, `i16`, `i32`, `i64`, `u8`, `u32`, `usize`, etc.)
* All floating-point numbers (`f32`, `f64`)
* Booleans (`bool`)
* Characters (`char`)
* Tuples containing **only** `Copy` types (e.g. `(i32, f64)` is Copy, but `(i32, String)` is **NOT** Copy!)
* Fixed arrays containing `Copy` types (e.g. `[i32; 5]`)

---

## 3. Which Values Move?
There is **no standard `Move` trait**. Moving is the default behavior for values whose type does not implement `Copy`, including:
* `String`
* `Vec<T>`
* `HashMap<K, V>`
* Custom `struct` and `enum` types (by default)

---

## 4. Moves in Function Calls: Giving Away Ownership

Ownership transfer doesn't just happen with `let y = x`.  
A by-value parameter moves a non-`Copy` argument or copies a `Copy` argument. Passing `&value` instead borrows it.

Look at this code carefully:

```rust
fn take_ownership(text: String) {
    println!("Function received: {}", text);
} // text goes out of scope and is DROPPED here!

fn make_copy(number: i32) {
    println!("Function received number: {}", number);
} // The parameter goes out of scope; the caller still has its own copy.

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
When the function returns, `text` is dropped and its buffer is released to the allocator. The moved-from `my_string` cannot be used; it is not an empty `String`. Cleanup does not promise secure erasure.

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
- [x] Primitive types such as `i32`, `f64`, `bool`, and `char` implement `Copy`.
- [x] Types such as `String` and `Vec<T>` do not implement `Copy`, so by-value assignment moves them.
- [x] When a value moves, the original variable becomes invalid immediately.
- [x] Passing a value to a function follows its type: `Copy` duplicates it; otherwise it moves.
- [x] Functions can return ownership back via return values.

---


## Symbols and Practical Boundaries

Copying a written number makes two notes; handing over a suitcase transfers responsibility for one suitcase.

| Symbol or term | Meaning |
|---|---|
| `Copy` | implicit duplication permission |
| `String` parameter | ownership transfer |
| `&String` | shared borrowing |

**Common pitfall:** No standard Move trait exists. A non-Copy struct can contain only numbers and still move; location does not decide Copy.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/06_ownership_and_memory/task_53_move_vs_copy/TASK.md`](../../../tasks/step_1_core_foundations/06_ownership_and_memory/task_53_move_vs_copy/TASK.md).
* Next lesson: [Lesson 54: Deep Copying with `.clone()`](54_clone_deep_copy.md).

Further reading: [Copy documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html).
