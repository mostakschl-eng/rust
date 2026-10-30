# Task 12: Gas Station Fuel Pump Calculator (Floating-Point `f64`)

## 🎯 What is Your Task?
Write a Rust program simulating a **Gas Station Fuel Pump** using `f64` decimals.

Follow these steps:
1. `price_per_gallon: f64` = `3.85` ($3.85 per gallon)
2. `gallons_pumped: f64` = `12.5` (12.5 gallons pumped into car)
3. Calculate `fuel_cost: f64` = `gallons_pumped * price_per_gallon`
4. Add a flat car wash discount of `$2.50`:
   `final_total: f64` = `fuel_cost - 2.50`
5. Print all values formatted nicely!

---

## 📋 Expected Output on Your Screen
```text
=== METRO GAS STATION ===
Price Per Gallon: $3.85
Gallons Pumped:   12.5
Fuel Subtotal:    $48.125
Car Wash Coupon:  -$2.5
Final Total Due:  $45.625
=========================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Declare variables using `f64` decimals.
3. Perform multiplication and subtraction.
4. Print the receipt.
5. Test with:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_12_floating_points
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

Use f64 values for both operands of the multiplication. The discount is subtracted after computing the fuel subtotal.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Print the calculated variables rather than fixed answer strings. Default float display may omit trailing zeros.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

