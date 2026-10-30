# Lesson 27: Returning Values from Loops (`break value;`)

Welcome to Lesson 27! 

Did you know that in Rust, a `loop` can **compute an answer and hand it directly back to you**?
This is one way Rust lets a search loop produce a value directly.
In Rust, you can write: **`break value;`**!

---

## 1. The Real-Life Analogy: The Deep-Sea Pearl Diver

Imagine a deep-sea diver diving underwater to find a precious black pearl:
* The diver swims in circles under the water (`loop`).
* They check shell after shell.
* When they finally spot the pearl, they grab it and **surface from the water holding the pearl in their hand**: `break pearl;`!
* The person waiting on the boat receives the pearl directly: `let prize = loop { ... break pearl; };`!

---

## 2. Returning a Value with `break`

Look at how elegant this Rust syntax is:

```rust
fn main() {
    let mut counter = 0;

    // The loop computes an answer and hands it to `result`:
    let result = loop {
        counter += 1;

        if counter == 10 {
            // When we reach 10, exit the loop AND return counter * 2!
            break counter * 2;
        }
    }; // Semicolon finishes the `let result = ...;` assignment

    println!("The loop computed the answer: {}", result);
}
```

### The Output on Screen:
```text
The loop computed the answer: 20
```

Notice:
1. `break counter * 2;` stops the loop **and** passes `20` out!
2. `result` receives `20` and stays **immutable** (no `mut` needed for `result`).

---

## 3. Why This is Useful: Retrying Failed Network Connections

Imagine your app is trying to connect to a server:
* You retry connecting in a loop.
* On attempt 3, the server responds with a session token.
* You write `break session_token;` to exit the retry loop and immediately store your session token!

---

## 4. Summary Checklist
- [x] In Rust, `break` can return a value: `break my_value;`.
- [x] The returned value can be captured directly with `let result = loop { ... };`.
- [x] Semicolon goes at the end of `};`.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/03_logic_and_loops/task_27_break_with_value/TASK.md`](../../../tasks/step_1_core_foundations/03_logic_and_loops/task_27_break_with_value/TASK.md)**.
1. Read the task requirements.
2. Build an annual-deposit calculator that loops until a savings account reaches or exceeds $1,000, then returns the total years taken with `break years;`.
3. Run `cargo run` and test returning values from loops!

## Symbol reference

| Symbol | Meaning |
|---|---|
| `break value;` | Exit a loop and supply its value |
| `let result = loop { ... };` | Bind the value produced by the loop |

## Common pitfalls and limits

Value-carrying break is used with loop, not for or while. Every break that supplies a value for the same loop must have a compatible type. Ensure the goal can actually be reached.
