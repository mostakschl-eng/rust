# Lesson 05: Calling Functions & How Code Flows in Time

Welcome to Lesson 05! 

Now that you know what a function is, let's understand **how the computer's brain moves through code in time**. This is called **Execution Flow**.

---

## 1. The Real-Life Analogy: Cooking with Recipe Cards

Imagine you are in a kitchen cooking dinner:
1. You are following the master recipe: **[Friday Dinner]**.
2. Step 1: Cook Rice.
3. Step 2: Make Curry. But wait! Making curry is complex, so the card says: *"See Recipe Card B: Spicy Chicken Curry"*.
4. You put down the master card, pick up **Card B**, cook the curry, and when it is finished, you **return right back** to where you left off on the master card!
5. Step 3: Serve on table.

This is exactly how Rust executes code!

---

## 2. Functions Calling Other Functions

A function doesn't just have to be called from `main`. Functions can call other functions!

Look at this daily morning routine example:

```rust
fn brush_teeth() {
    println!("Step 1: Brushing teeth cleanly...");
}

fn eat_breakfast() {
    println!("Step 2: Eating breakfast...");
}

fn morning_routine() {
    println!("--- Morning routine starting ---");
    brush_teeth();   // Function calls brush_teeth!
    eat_breakfast(); // Function calls eat_breakfast!
    println!("--- Morning routine complete ---");
}

fn main() {
    println!("Alarm ringing: 7:00 AM");
    morning_routine(); // Main calls morning_routine!
    println!("Leaving house for school or work!");
}
```

### The Output on Screen:
```text
Alarm ringing: 7:00 AM
--- Morning routine starting ---
Step 1: Brushing teeth cleanly...
Step 2: Eating breakfast...
--- Morning routine complete ---
Leaving house for school or work!
```

---

## 3. Tracing the Computer's Path (The Execution Journey)

Let's watch the computer's exact steps, second by second:

| Step | Line | What the Computer Does |
|---|---|---|
| 1 | `fn main()` | The program starts at the main door. |
| 2 | `println!("Alarm ringing: 7:00 AM");` | Prints the alarm text. |
| 3 | `morning_routine();` | **JUMP!** Computer pauses `main` and jumps up into `morning_routine`. |
| 4 | `println!("--- Morning routine starting ---");` | Prints starting banner. |
| 5 | `brush_teeth();` | **JUMP AGAIN!** Computer pauses `morning_routine` and jumps into `brush_teeth`. |
| 6 | `println!("Step 1: Brushing teeth cleanly...");` | Prints teeth brushing text. |
| 7 | `}` (end of `brush_teeth`) | **RETURN!** Jumps back to `morning_routine` at line 14. |
| 8 | `eat_breakfast();` | **JUMP!** Jumps into `eat_breakfast`. |
| 9 | `println!("Step 2: Eating breakfast...");` | Prints breakfast text. |
| 10 | `}` (end of `eat_breakfast`) | **RETURN!** Jumps back to `morning_routine` at line 15. |
| 11 | `println!("--- Morning routine complete ---");` | Prints routine finished banner. |
| 12 | `}` (end of `morning_routine`) | **RETURN!** Jumps all the way back to `main` at line 20. |
| 13 | `println!("Leaving house for school or work!");` | Prints final departure line. |
| 14 | `}` (end of `main`) | Entire program exits. Done! |

---

## 4. Why This Architecture Matters

This is how large games and operating systems work!
Instead of one giant 5,000-line messy file, programmers break work down into small, neat, 5-line functions that call each other logically.

---

## 5. Summary Checklist
- [x] Code runs line-by-line, top-to-bottom inside `main()`.
- [x] Calling a function pauses the current function, jumps into the called function, and returns when done.
- [x] Functions can call other functions to keep complex tasks organized.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/01_baby_steps/task_05_execution_flow/TASK.md`](file:///d:/rust-learning/tasks/01_baby_steps/task_05_execution_flow/TASK.md)**.
1. Read the task requirements.
2. Build a three-stage Rocket Launch countdown program using nested functions.
3. Run `cargo run` and watch the execution flow in action!
