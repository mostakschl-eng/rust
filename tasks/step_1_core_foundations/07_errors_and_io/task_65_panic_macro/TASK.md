# Task 65: Rocket Launch Controller (Unrecoverable Errors with panic!)

## 🎯 What is Your Task?
Write a Rust program to simulate a launch controller that halts with `panic!` if life-critical conditions fail:
1. Define a function `launch_sequence(fuel_level: u8, countdown: i32)`:
   - Check `if fuel_level < 20`: immediately call:
     ```rust
     panic!("EMERGENCY ABORT: Fuel level critical ({}%)!", fuel_level);
     ```
   - Print `"Countdown reached T-minus {} seconds. Fuel: {}%."`
   - Print `"Engines ignite! Liftoff confirmed! 🚀"`
2. In `main()`:
   - Run a successful launch sequence with `fuel_level: 95` and `countdown: 0`.
   - Leave a clear instructions comment on how to test the emergency abort panic!

---

## 📋 Expected Output on Your Screen
```text
=== ROCKET LAUNCH CONTROL CENTER ===
Performing pre-flight safety check...
Countdown reached T-minus 0 seconds. Fuel: 95%.
Engines ignite! Liftoff confirmed! 🚀
====================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Write `launch_sequence` using `panic!` for fatal conditions.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/07_errors_and_io/task_65_panic_macro
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Check the low-fuel branch before printing any successful countdown or ignition.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Call the function with 95 for the successful example; leave a commented call with 10 to explore the panic path.

</details>

**Before running:** Panic is for failed assumptions in this toy example. Low fuel is normally an operational condition handled by Result; real safety controllers require verified shutdown behavior.
