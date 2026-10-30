# Lesson 65: Unrecoverable Errors: The `panic!` Macro

In Lesson 64, we handled **recoverable errors** using `Result<T, E>`.  
When a user types an invalid password or a file is missing, you don't want your program to crash! You want to display an error and let them try again.

However, sometimes a program encounters a catastrophic situation where **continuing would violate an internal assumption**:
* Essential operating system files are corrupt.
* A critical cryptographic invariant was violated.
* An array index out of bounds bug in internal logic was triggered.

In Rust, this is where we pull the emergency brake: **`panic!`**! 🚨💥

---

## 1. The Real-Life Analogy: The Red Emergency Stop Button 🛑

Think of a high-speed industrial assembly line or subway train:
* If a package is slightly misaligned: A sensor beeps, a robotic arm nudges it back (**Recoverable Error: `Result`**).
* But what if a steel beam collapses onto the railway tracks?
* You don't try to drive through it. You hit the giant red **EMERGENCY STOP** button!
* The motors shut off instantly, the alarms blare, and the train halts to prevent disaster!

In Rust:
> The **`panic!`** macro is Rust's emergency stop button.  
> It invokes the panic hook, then normally unwinds the current thread. An abort configuration ends the process without unwinding. A panic is not a hardware shutdown or a guarantee of external safety.

---

## 2. Triggering a Manual `panic!`

You can call `panic!` directly anywhere in your code:

```rust
fn main() {
    println!("System initializing...");

    let critical_hardware_connected = false;

    if !critical_hardware_connected {
        panic!("FATAL: Required encryption hardware dongle not detected!");
    }

    println!("System online!"); // Never reached!
}
```

### 📋 Expected Terminal Output:
```text
System initializing...
thread 'main' panicked at src/main.rs:7:9:
FATAL: Required encryption hardware dongle not detected!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The program printed the exact file and line number (`src/main.rs:7:9`) and stopped safely!

---

## 3. What Happens During a Panic? (Unwinding vs Aborting)

By default, when a panic occurs, Rust performs **Stack Unwinding**:
1. Rust walks back up the stack frame by frame.
2. It drops initialized local values in the unwound frames, releasing resources according to their types.
3. An uncaught panic ends that thread; in the main thread this normally ends the process with failure. Other threads can observe a worker panic through `join()`.

*(In resource-constrained embedded systems, you can tell Cargo to immediately `abort` without unwinding to keep the binary size as small as possible).*

---

## 4. Reading Stack Backtraces with `RUST_BACKTRACE=1`

If a panic happens deep inside a library, you can see the exact function call stack by setting the environment variable:

In PowerShell:
```powershell
$env:RUST_BACKTRACE=1; cargo run
```

This will print a complete step-by-step diagnostic trace of every function called leading up to the panic!

---

## 5. When to Panic vs When to Return a Result

| Scenario | Recommendation |
|---|:---:|
| User input error (bad password, invalid date) | Return `Result::Err` 🛡️ |
| Expected network drop or missing config | Return `Result::Err` 🛡️ |
| Broken contract (e.g. function called with impossible internal state) | `panic!` 🚨 |
| Example code, unit tests, quick prototypes | `panic!` / `.unwrap()` 🧪 |

---

## 6. Quick Recap Checklist
- [x] `panic!("message")` stops normal execution and starts panic handling.
- [x] Used exclusively for unrecoverable errors and internal contract violations.
- [x] The usual strategy unwinds; abort skips destructors. Neither guarantees a safe state in external equipment.
- [x] Set `RUST_BACKTRACE=1` to view the full call stack trace.
- [x] Always prefer returning `Result` for expected operational failures!

---


## Symbols and Practical Boundaries

A software emergency brake stops normal execution. It cannot by itself turn off a real engine or repair a corrupted file.

| Symbol or term | Meaning |
|---|---|
| `panic!(...)` | trigger panic handling |
| `unwind` | drop stack locals |
| `abort` | end process without unwind |

**Common pitfall:** Panic is for failed assumptions in this toy example. Low fuel is normally an operational condition handled by Result; real safety controllers require verified shutdown behavior.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/07_errors_and_io/task_65_panic_macro/TASK.md`](../../../tasks/step_1_core_foundations/07_errors_and_io/task_65_panic_macro/TASK.md).
* Next lesson: [Lesson 66: Propagating Errors: The `?` (Question Mark) Operator](66_question_mark_operator.md).
