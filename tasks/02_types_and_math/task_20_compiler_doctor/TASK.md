# Task 20: The Compiler Doctor (Fixing 3 Intentional Bugs)

## 🎯 What is Your Task?
You are the **Compiler Doctor**! 
The starter code in `src/main.rs` contains **3 intentional errors** that prevent it from compiling:

1. **Bug 1:** An immutable variable is assigned a new value without `mut`.
2. **Bug 2:** An integer is multiplied directly by an `f64` float without type casting.
3. **Bug 3:** A tuple is printed using `{}` instead of `{:?}`.

Your mission is to find and fix all three bugs so that `cargo run` succeeds and prints the final report!

---

## 📋 Expected Output on Your Screen
```text
=== COMPILER DOCTOR: PATIENT CURED ===
Prescription Count: 3
Price Per Pill:     $1.50
Total Medicine:     $4.50
Doctor Credentials: ("Dr. Rahim", 101)
Status:             Healthy & Compiling!
======================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_20_compiler_doctor/src/main.rs).
2. Run `cargo check` in your terminal to see Rust's error messages.
3. Read the line numbers and error codes.
4. Apply the fixes:
   - Add `mut` where needed.
   - Cast with `as f64`.
   - Change `{}` to `{:?}` for the tuple.
5. Run:
   ```bash
   cd tasks/02_types_and_math/task_20_compiler_doctor
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: The 3 cures</b></summary>

* Cure 1: `let mut prescription_count = 1;`
* Cure 2: `(prescription_count as f64) * price_per_pill`
* Cure 3: `println!("Doctor Credentials: {:?}", doctor_badge);`
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_20_compiler_doctor/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_20_compiler_doctor/solution/explain.md)
