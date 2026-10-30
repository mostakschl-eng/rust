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
> Regular `fn` items cannot capture local variables from an enclosing function. They can access items such as constants, statics, and other functions.

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

1. **`FnOnce`:** Every closure supports being consumed for a call. A closure that moves a captured non-`Copy` value out of its body may support only this trait.
2. **`FnMut`:** Also supports repeated calls through mutable access, provided the body does not move captured values out. It may change captured state.
3. **`Fn`:** Also supports calls through shared access, when the body neither mutates captured state in a way requiring exclusive access nor moves it out.

These traits overlap: `Fn` implies `FnMut` and `FnOnce`; `FnMut` implies `FnOnce`. Capture mode and call capability are different questions. A `move` closure can own a `String` and still be called repeatedly if it only reads it.

### Example of `move` Closures:
If you pass a closure to a new background thread, use the **`move`** keyword to force the closure to take ownership of the environment:
```rust
let text = String::from("thread message");
let worker = move || println!("{}", text); // text is moved into worker
worker();
worker(); // Still valid: the body only borrows its owned String for printing
```

---

## 5. Quick Recap Checklist
- [x] Closures are anonymous inline functions defined with `|param| body`.
- [x] Type annotations are optional when the compiler can infer them.
- [x] Closures can capture variables from the surrounding environment.
- [x] Closures implement one or more call traits according to what their bodies do.
- [x] Use `move ||` to capture by value: non-`Copy` values move and `Copy` values copy. Moving a reference does not extend the lifetime of its referent.

---


## Symbols and Practical Boundaries

A cashier note can remember the current discount and tax rates, so each price need not carry a separate copy of those instructions.

| Symbol or term | Meaning |
|---|---|
| `|x| body` | closure |
| `||` | no parameters |
| `move` | capture by value |
| `final expression` | returned value |

**Common pitfall:** Fn, FnMut, and FnOnce overlap. move chooses capture mode, not a one-call limit. Floating-point prices here teach closures, not exact accounting.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/08_advanced_and_concurrency/task_76_closures/TASK.md`](../../../tasks/step_1_core_foundations/08_advanced_and_concurrency/task_76_closures/TASK.md).
* Next lesson: [Lesson 77: Iterators: Transforming Data (`map`, `filter`, `collect`)](77_iterators.md).

Further reading: [Rust Book: closures](https://doc.rust-lang.org/book/ch13-01-closures.html).
