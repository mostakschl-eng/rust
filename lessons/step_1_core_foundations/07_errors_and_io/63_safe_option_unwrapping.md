# Lesson 63: Safe Option Unwrapping (`unwrap_or`, `expect`, `match`)

In Lesson 62, we saw that values can be wrapped inside an `Option<T>` box.  
Now, how do we open that box and extract the inner value `T` so we can use it?

There are dangerous ways (that can crash your software!) and safe, battle-tested ways.  
The right choice depends on what absence means. A theme may have a default; a missing payment amount should usually be reported.

---

## 1. The Real-Life Analogy: The Locked Padlock 🔒

Imagine you receive a locked briefcase:
* **The Dangerous Way (`.unwrap()`):** You smash the briefcase open with a sledgehammer. If there was a gift inside, great! But if the briefcase was empty, the hammer hits a bomb and blows up your entire house (**Program Panic & Crash**)!
* **The Safe Way (`.unwrap_or(default)`):** You open the briefcase. If there's money inside, take it! If it's empty, pull out your own $10 bill from your pocket as a fallback!
* **The Structured Way (`match`):** You inspect the briefcase methodically and handle both outcomes with tailored code.

---

## 2. The Dangerous Trap: `.unwrap()` ⚠️

Rust provides a method called `.unwrap()`:
```rust
let present: Option<&str> = None;
let gift = present.unwrap(); // 💥 CRASH: thread 'main' panicked at 'called Option::unwrap() on a None value'
```
> [!CAUTION]
> Use `.unwrap()` only where a present value is established by an invariant.  
> On `None`, it panics in the current thread; an uncaught main-thread panic normally ends the program.

---

## 3. The Better Panic: `.expect("custom error message")`

If you are 100% certain a value must exist (or if crashing is genuinely the only logical choice), use **`.expect("message")`** instead of `.unwrap()`.  
It crashes, but it prints your custom diagnostic message explaining **why** it failed:

```rust
fn main() {
    let database_port: Option<u16> = Some(5432);

    let port = database_port.expect("CRITICAL: Database port must be set in configuration!");
    println!("Connecting to database on port: {}", port);
}
```

---

## 4. A Deliberate Fallback: `.unwrap_or(default_value)`

Use a **default value** only when the application explicitly allows one. Silently replacing missing required information can hide a real problem. For an optional theme, `.unwrap_or()` is reasonable:

```rust
fn main() {
    let user_theme: Option<&str> = None; // User hasn't chosen a dark or light theme yet!

    // If user_theme is Some, use it; if None, fall back to "Light Mode":
    let active_theme = user_theme.unwrap_or("Light Mode");

    println!("Active Theme: {}", active_theme);
}
```

### 📋 Expected Terminal Output:
```text
Active Theme: Light Mode
```

This expression handles `None` without panicking. It does not guarantee that the rest of the program cannot fail.

---

## 5. Lazy Defaults with `.unwrap_or_else()`

What if calculating your fallback default value takes a long time or costs CPU (like querying a database)?  
Use **`.unwrap_or_else(|| { ... })`**:
* The closure inside `||` is **only executed if the Option is actually `None`**!

```rust
fn calculate_expensive_fallback() -> i32 {
    println!("Computing complex fallback...");
    100
}

fn main() {
    let fast_cache: Option<i32> = Some(42);

    // Because fast_cache is Some(42), calculate_expensive_fallback() is NEVER called!
    let val = fast_cache.unwrap_or_else(calculate_expensive_fallback);

    println!("Final value: {}", val);
}
```

---

## 6. Summary Comparison: Unwrapping Strategies

| Method | Behavior on `Some(v)` | Behavior on `None` | Production Rating |
|---|---|---|:---:|
| `.unwrap()` | Returns `v` | Panics | Established invariant or a small example |
| `.expect(msg)` | Returns `v` | Panics with a message | Explain why the value must exist |
| `.unwrap_or(default)` | Returns `v` | Returns `default` safely 🛡️ | Only when fallback is correct |
| `.unwrap_or_else(fn)` | Returns `v` | Calls fallback function lazily ⚡ | Only when fallback is correct |
| `match opt { ... }` | Handled by arm | Handled by arm 🎯 | Handle each case explicitly |

---

## 7. Quick Recap Checklist
- [x] `.unwrap()` crashes with panic if the value is `None`.
- [x] `.expect("message")` crashes with a descriptive message.
- [x] `.unwrap_or(default)` supplies a fallback on `None`; its argument is evaluated even for `Some`.
- [x] `.unwrap_or_else(|| ...)` computes fallback values lazily.
- [x] Choose a fallback, an explicit missing-value branch, or propagation according to the application; never hide a required value by inventing one.

---


## Symbols and Practical Boundaries

A missing paint color can use the shop default. A missing identity document cannot be replaced with a made-up identity.

| Symbol or term | Meaning |
|---|---|
| `.unwrap_or(v)` | eager fallback |
| `.unwrap_or_else(f)` | lazy fallback |
| `.expect(msg)` | panic if absent |

**Common pitfall:** expect is a presence assertion, not certificate verification. This exercise uses a fixed Some value; real external certificate errors need deliberate error handling.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/07_errors_and_io/task_63_safe_option_unwrapping/TASK.md`](../../../tasks/step_1_core_foundations/07_errors_and_io/task_63_safe_option_unwrapping/TASK.md).
* Next lesson: [Lesson 64: Recoverable Errors: The `Result<T, E>` Enum](64_result_enum.md).
