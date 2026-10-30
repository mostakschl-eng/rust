# Lesson 76: Closures: Anonymous Inline Functions (`|x| x + 1`)

Sometimes, you need a quick, small function to calculate something on the fly:
* Doubling a number in a list.
* Sorting items by price.
* Passing a small callback to a background thread.

Defining a full `fn my_helper_function(...) { ... }` with a formal name, full type annotations, and explicit signatures can feel heavy for a one-line task.

In Rust, small anonymous inline functions are called **Closures**! 🎯⚡

---

## 1. The Real-Life Analogy: The Disposable Sticky Note Formula 📝

* **Standard `fn`:** A laminated recipe book bound in leather and stored in the library. It has an official title, table of contents, and strict instructions.
* **A Closure:** A quick handwritten formula scribbled on a sticky note: *"Take whatever number they give you, and multiply by 2"*. You stick it to the cashier's desk, use it for 5 minutes, and throw it away!

---

## 2. Closure Syntax: Pipes `|x|` Instead of Parentheses `(x)`

Closures use vertical pipes **`|...|`** to define parameters:

```rust
fn main() {
    // 1. A closure that adds 1:
    let add_one = |x: i32| x + 1;

    // 2. Calling a closure looks identical to a function:
    let result = add_one(5);
    println!("5 + 1 = {}", result);

    // 3. Multi-line closure with curly braces {}:
    let calculate_tax = |amount: f64| {
        let tax_rate = 0.15;
        amount * tax_rate
    };

    println!("Tax on $200: ${:.2}", calculate_tax(200.0));
}
```

### 📋 Expected Terminal Output:
```text
5 + 1 = 6
Tax on $200: $30.00
```

---

## 3. The Superpower: Capturing the Environment! 🌐

Here is the biggest difference between a regular `fn` and a closure:
> **Closures can capture and read variables from their surrounding environment!**  
> Regular functions `fn` can NEVER access variables from outside their body.

Look at this code:

```rust
fn main() {
    let multiplier = 10; // Variable in outer scope

    // The closure captures 'multiplier' from outside!
    let multiply_by_outer = |n: i32| n * multiplier;

    println!("Result: {}", multiply_by_outer(5)); // 50
}
```

Try doing that with a regular `fn`—it's impossible!

---

## 4. The Three Closure Traits (`Fn`, `FnMut`, `FnOnce`)

Rust automatically infers how a closure captures variables based on what it does:

1. **`Fn` (Shared Borrow):** Captures variables by immutable reference `&T`. Can be called multiple times without modifying anything.
2. **`FnMut` (Mutable Borrow):** Captures variables by mutable reference `&mut T`. Modifies outer variables.
3. **`FnOnce` (Takes Ownership):** Takes ownership of captured variables. Can only be called **once** because it consumes the data!

### Example of `move` Closures:
If you pass a closure to a new background thread, use the **`move`** keyword to force the closure to take ownership of the environment:
```rust
let text = String::from("thread message");
let worker = move || println!("{}", text); // text is moved into worker!
```

---

## 5. Quick Recap Checklist
- [x] Closures are anonymous inline functions defined with `|param| body`.
- [x] Type annotations are optional when the compiler can infer them.
- [x] Closures can capture variables from the surrounding environment.
- [x] Automatically classified into `Fn`, `FnMut`, or `FnOnce`.
- [x] Use `move ||` to transfer ownership of captured variables.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/08_advanced_and_concurrency/task_76_closures/TASK.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_76_closures/TASK.md).
* Next lesson: [Lesson 77: Iterators: Transforming Data (`map`, `filter`, `collect`)](file:///d:/rust-learning/lessons/08_advanced_and_concurrency/77_iterators.md).
