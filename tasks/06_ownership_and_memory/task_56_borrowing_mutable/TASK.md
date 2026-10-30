# Task 56: Emergency Grid Operator (Mutable Borrowing &mut)

## 🎯 What is Your Task?
Write a Rust program to mutate strings and primitive floats in place using exclusive mutable references (`&mut`):
1. Define a function `add_alert_badge(msg: &mut String)`:
   - Appends `" [CRITICAL LEVEL 5 ALERT 🚨]"` to `msg`.
2. Define a function `boost_voltage(voltage: &mut f64, multiplier: f64)`:
   - Multiplies the dereferenced voltage by `multiplier` (`*voltage *= multiplier;`).
3. In `main()`:
   - Declare `let mut system_status = String::from("Cooling Pump Fault");`
   - Declare `let mut grid_voltage = 220.0;`
   - Print original values.
   - Call `add_alert_badge(&mut system_status);`
   - Call `boost_voltage(&mut grid_voltage, 1.25);`
   - Print updated values.

---

## 📋 Expected Output on Your Screen
```text
=== POWER GRID CONTROL CENTER ===
Original Status:  Cooling Pump Fault
Original Voltage: 220.00 V

Applying emergency protocols...
Updated Status:   Cooling Pump Fault [CRITICAL LEVEL 5 ALERT 🚨]
Updated Voltage:  275.00 V
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_56_borrowing_mutable/src/main.rs).
2. Use `&mut String` for text modification.
3. Use `*voltage *= multiplier;` to dereference and modify primitive floats.
4. Pass `&mut system_status` and `&mut grid_voltage`.
5. Test:
   ```bash
   cd tasks/06_ownership_and_memory/task_56_borrowing_mutable
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_56_borrowing_mutable/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/06_ownership_and_memory/task_56_borrowing_mutable/solution/explain.md)

