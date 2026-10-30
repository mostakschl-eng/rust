# Task 20: Solution Breakdown & Compiler Diagnostic Cures

## 1. The 3 Diagnostic Cures Explained

1. **`let mut prescription_count = 1;`:**
   Adding `mut` satisfies error `E0384` (*cannot assign twice to immutable variable*).

2. **`(prescription_count as f64) * price_per_pill;`:**
   Explicit casting satisfies error `E0308` (*mismatched types: cannot multiply integer by float*).

3. **`println!("Doctor Credentials: {:?}", doctor_badge);`:**
   Using debug formatting `{:?}` satisfies error `E0277` (*`(&str, i32)` cannot be formatted with the default formatter*).

Congratulations on completing all 20 lessons and tasks across Level 1 and Level 2!

