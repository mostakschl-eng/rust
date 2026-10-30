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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_64_result_enum/src/main.rs).
2. Implement `split_bill` returning `Ok` or `Err`.
3. Handle with `match` and `.unwrap_or()`.
4. Test:
   ```bash
   cd tasks/07_errors_and_io/task_64_result_enum
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_64_result_enum/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_64_result_enum/solution/explain.md)

