# Task 20: The Compiler Doctor (Fixing 3 Intentional Bugs)

## 🎯 What is Your Task?
You are the **Compiler Doctor**! 
The starter code in `src/main.rs` contains **3 intentional errors** that prevent it from compiling:

1. **Bug 1:** An immutable variable is assigned a new value without `mut`.
2. **Bug 2:** An integer is multiplied directly by an `f64` float without type casting.
3. **Bug 3:** A tuple is printed using `{}` instead of `{:?}`.

This starter intentionally fails `cargo check`: it is the exception to the usual compiling TODO starters. The compiler may reveal some errors only after earlier ones are repaired.

Your mission is to find and fix all three bugs so that `cargo run` succeeds and prints the final report!

---

## 📋 Expected Output on Your Screen
```text
=== COMPILER DOCTOR: PATIENT CURED ===
Initial Prescription: 1
Prescription Count: 3
Price Per Pill:     $1.50
Total Medicine:     $4.50
Doctor Credentials: ("Dr. Rahim", 101)
Status:             Healthy & Compiling!
======================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Run `cargo check` in your terminal to see Rust's error messages.
3. Read the line numbers and error codes.
4. Apply the fixes:
   - Add `mut` where needed.
   - Cast with `as f64`.
   - Change `{}` to `{:?}` for the tuple.
5. Run:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_20_compiler_doctor
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Run cargo check and fix one diagnostic at a time. Look for the binding that is assigned again, the unlike numeric types, and the tuple formatter.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Make the reassigned binding mutable; convert the integer operand before float multiplication; choose the developer formatter for the tuple.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

