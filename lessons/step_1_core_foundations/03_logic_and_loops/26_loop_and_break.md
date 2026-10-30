# Lesson 26: Infinite Loops & Early Exits (`loop` and `break`)

Welcome to Lesson 26! 

Computers never get tired. If you ask a human to count from 1 to 1,000,000, they will get exhausted.
A computer can repeat simple work quickly; actual timing depends on the work and hardware, especially terminal output.

In programming, repeating instructions over and over is called **Looping**.
Today, we start with Rust's most direct looping tool: the **`loop`** statement!

---

## 1. The Real-Life Analogy: The Hamster Wheel & The Emergency Brake

Imagine a hamster running on an exercise wheel:
* The wheel keeps spinning round and round forever: `loop { spin(); }`.
* If there is no exit, the wheel will spin for eternity!
* But when the hamster gets tired, it hits the **Emergency Brake (`break`)** and hops off the wheel!

> In Rust:  
> `loop { ... }` runs the code inside **forever and ever**.  
> The keyword **`break;`** is your emergency brake that instantly terminates the loop!

---

## 2. A Simple Counter with `loop` and `break`

Look at how we count up to 5 and then stop:

```rust
fn main() {
    let mut count = 1;

    println!("Starting count-up...");

    loop {
        println!("Count: {}", count);

        if count == 5 {
            println!("Reached 5! Hitting the brakes! 🛑");
            break; // <-- Halts the loop immediately!
        }

        count += 1; // Increase count by 1
    }

    println!("Loop finished! Program continues safely.");
}
```

### The Output on Screen:
```text
Starting count-up...
Count: 1
Count: 2
Count: 3
Count: 4
Count: 5
Reached 5! Hitting the brakes! 🛑
Loop finished! Program continues safely.
```

---

## 3. The `continue` Keyword (Skip to Next Lap)

What if you want to skip just *one* specific lap, but keep running the rest?
The keyword **`continue`** tells Rust:
> *"Stop right now, skip the rest of this lap, and immediately start the next lap!"*

```rust
fn main() {
    let mut num = 0;

    loop {
        num += 1;

        if num > 6 {
            break; // Stop completely after 6
        }

        if num == 3 {
            println!("Skipping unlucky number 3! 🙈");
            continue; // Skips printing the normal line below!
        }

        println!("Processed number: {}", num);
    }
}
```

### The Output on Screen:
```text
Processed number: 1
Processed number: 2
Skipping unlucky number 3! 🙈
Processed number: 4
Processed number: 5
Processed number: 6
```

---

## 4. Common Beginner Traps

### ❌ Trap 1: The Infinite Freeze (Forgetting `break`)
```rust
let mut x = 0;
loop {
    println!("{}", x);
    // Forgot `break`!
}
```
* **What happens:** The program keeps printing `0` until interrupted or another failure stops it. Output speed depends on the terminal and system.
* **How to rescue your terminal:** Press **`Ctrl + C`** on your keyboard to kill the program!

---

## 5. Summary Checklist
- [x] `loop { ... }` executes instructions in a continuous cycle.
- [x] `break;` immediately exits the loop.
- [x] `continue;` skips the rest of the current iteration and starts the next lap.
- [x] Always ensure your loop has a reachable condition to trigger `break`!

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/03_logic_and_loops/task_26_loop_and_break/TASK.md`](../../../tasks/step_1_core_foundations/03_logic_and_loops/task_26_loop_and_break/TASK.md)**.
1. Read the task requirements.
2. Build an electric car battery recharge simulation that loops until battery reaches 100%.
3. Run `cargo run` and watch the recharge loop!

## Symbol reference

| Symbol | Meaning |
|---|---|
| `loop` | Repeat the body until control leaves |
| `break` | Exit the nearest loop |
| `continue` | Start the next iteration |
