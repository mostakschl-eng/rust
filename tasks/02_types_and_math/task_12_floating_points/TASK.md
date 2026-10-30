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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_12_floating_points/src/main.rs).
2. Declare variables using `f64` decimals.
3. Perform multiplication and subtraction.
4. Print the receipt.
5. Test with:
   ```bash
   cd tasks/02_types_and_math/task_12_floating_points
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Floats syntax</b></summary>

```rust
let price_per_gallon = 3.85;
let gallons_pumped = 12.5;
let fuel_cost = price_per_gallon * gallons_pumped;
let final_total = fuel_cost - 2.50;
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_12_floating_points/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_12_floating_points/solution/explain.md)

