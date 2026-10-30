# Task 64: Restaurant Bill Splitter (Result Enum)

## 🎯 What is Your Task?
Write a Rust program to safely split restaurant dinner checks using `Result<T, E>`:
1. Define a function:
   ```rust
   fn split_bill(total_amount: f64, people_count: u32) -> Result<f64, String>
   ```
   - If `people_count == 0`: return `Err(String::from("Cannot divide bill among 0 diners!"))`.
   - Else: return `Ok(total_amount / people_count as f64)`.
2. In `main()`:
   - Attempt 1: Split `$120.00` among 4 diners using `match`.
     `"Bill split successfully: $30.00 per person"`
   - Attempt 2: Split `$50.00` among 0 diners using `match`.
     `"⚠️ Calculation Rejected: Cannot divide bill among 0 diners!"`
   - Attempt 3: Split `$80.00` among 0 diners using `.unwrap_or(0.0)`.
     `"Fallback calculation: $0.00"`

---

## 📋 Expected Output on Your Screen
```text
=== RESTAURANT BILL SPLITTER ===
Attempt 1 (Valid Group):
Bill split successfully: $30.00 per person

Attempt 2 (Zero Group):
⚠️ Calculation Rejected: Cannot divide bill among 0 diners!

Attempt 3 (Fallback Unwrapping):
Fallback calculation: $0.00
================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Implement `split_bill` returning `Ok` or `Err`.
3. Handle with `match` and `.unwrap_or()`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/07_errors_and_io/task_64_result_enum
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Reject zero diners before performing the division.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Wrap a valid quotient in Ok, put the reason in Err, and match both variants before demonstrating the explicitly limited fallback.

</details>

**Before running:** A fallback of zero demonstrates a method; it is not a correct payable share for a failed bill calculation. Real billing should report the error. The exercise assumes finite nonnegative amounts; f64 also has rounding limits.
