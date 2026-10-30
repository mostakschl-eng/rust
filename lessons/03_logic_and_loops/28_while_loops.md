# Lesson 28: Conditional Repetition: `while` Loops

Welcome to Lesson 28! 

In Lesson 26, we used `loop` and had to manually write an `if` statement with a `break` to stop it.
What if we want a loop that **automatically tests a condition before every lap**?

Enter the **`while` loop**!

---

## 1. The Real-Life Analogy: The Washing Machine

Imagine your automatic washing machine doing a spin cycle:
* The machine rule is: **"While clothes are still wet, keep spinning!"**
* Before every spin, it checks moisture.
* Spin 1: Still wet? Yes. Spin again.
* Spin 2: Still wet? Yes. Spin again.
* Spin 3: Still wet? **No (False)!** The machine stops automatically and beeps!

> In Rust:  
> `while condition { ... }` runs the code inside **as long as the condition remains `true`**.  
> The moment the condition becomes `false`, the loop stops automatically!

---

## 2. A Rocket Countdown with `while`

Look at how concise a countdown timer is with `while`:

```rust
fn main() {
    let mut countdown = 3;

    println!("Starting launch countdown:");

    while countdown > 0 {
        println!("T-minus {}...", countdown);
        countdown -= 1; // Decrement by 1!
    }

    println!("🚀 BLASTOFF! We have liftoff!");
}
```

### The Output on Screen:
```text
Starting launch countdown:
T-minus 3...
T-minus 2...
T-minus 1...
🚀 BLASTOFF! We have liftoff!
```

---

## 3. How the Computer Runs a `while` Loop

| Lap | Is `countdown > 0`? | Action Taken | Next `countdown` Value |
|---|---|---|---|
| Lap 1 | `3 > 0` -> **true** | Prints `T-minus 3...` | `3 - 1 = 2` |
| Lap 2 | `2 > 0` -> **true** | Prints `T-minus 2...` | `2 - 1 = 1` |
| Lap 3 | `1 > 0` -> **true** | Prints `T-minus 1...` | `1 - 1 = 0` |
| Lap 4 | `0 > 0` -> **FALSE!** | **Loop terminates immediately!** | Exits to BLASTOFF line |

---

## 4. What If the Condition is False from the Beginning?

If the condition is `false` when Rust first encounters the `while` loop, **the loop will run ZERO times**:

```rust
let fuel = 0;
while fuel > 0 {
    println!("Flying plane..."); // Never runs because fuel is 0!
}
println!("Plane cannot take off without fuel!");
```

---

## 5. Summary Checklist
- [x] `while condition { ... }` checks before every single lap.
- [x] The loop runs as long as the condition evaluates to `true`.
- [x] Always make sure the variable inside the condition changes, otherwise the loop will never end!

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/03_logic_and_loops/task_28_while_loops/TASK.md`](file:///d:/rust-learning/tasks/03_logic_and_loops/task_28_while_loops/TASK.md)**.
1. Read the task requirements.
2. Build an ATM PIN brute-force lockout simulator that counts 3 failed attempts using a `while` loop.
3. Run `cargo run` and test your while loop!
