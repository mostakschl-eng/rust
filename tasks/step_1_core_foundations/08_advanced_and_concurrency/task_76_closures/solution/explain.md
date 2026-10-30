# Detailed Explanation: Task 76 (Closures)

---

### 1. Defining Single-Line Closures
```rust
let apply_discount = |price: f64| price * (1.0 - discount_rate);
```
* **Pipe Syntax `|price: f64|`**: Defines the parameter list, just like `(price: f64)` in a standard function.
* **Environment Capture**: Notice that `discount_rate` is defined *outside* the closure in `main()`, but `apply_discount` can read it directly! A standard `fn` cannot do this without passing `discount_rate` as a parameter.

---

### 2. Multi-Line Closures
```rust
let format_bill = |raw_price: f64| {
    let discounted = apply_discount(raw_price);
    apply_tax(discounted)
};
```
* When closure logic spans multiple statements, wrap the body in curly braces `{ ... }`.
* Closures can even call other closures!
* The final expression `apply_tax(discounted)` has no semicolon, returning the final `f64` value.


## Reading the Result Carefully

A cashier note can remember the current discount and tax rates, so each price need not carry a separate copy of those instructions.

| Symbol or term | Meaning |
|---|---|
| `|x| body` | closure |
| `||` | no parameters |
| `move` | capture by value |
| `final expression` | returned value |

Fn, FnMut, and FnOnce overlap. move chooses capture mode, not a one-call limit. Floating-point prices here teach closures, not exact accounting.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
