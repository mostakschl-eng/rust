# Task 19: Flight Telemetry Debugger (Debug Printing `{:?}` & `{:#?}`)

## 🎯 What is Your Task?
Write a Rust program simulating an **Airplane Flight Telemetry Diagnostic Tool** using developer debug printing.

Requirements:
1. Declare flight coordinates as a tuple:
   `let flight_location = ("Flight 707", 32.7767, -96.7970, 35000);`
   *(Flight number, Latitude, Longitude, Altitude in feet)*
2. Print the flight location using **Standard Debug (`{:?}`)**.
3. Print the flight location using **Multi-line Pretty Debug (`{:#?}`)**.
4. Use the `dbg!` macro on an airspeed calculation:
   `let airspeed_knots = 480 + 25;`
   `dbg!(airspeed_knots);`

---

## 📋 Expected Output on Your Screen
```text
=== FLIGHT TELEMETRY DEBUGGER ===
Standard Debug: ("Flight 707", 32.7767, -96.797, 35000)
Pretty Debug:
(
    "Flight 707",
    32.7767,
    -96.797,
    35000,
)
[src/main.rs:16:5] airspeed_knots = 505
=================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_19_debug_printing/src/main.rs).
2. Declare `flight_location = ("Flight 707", 32.7767, -96.7970, 35000);`.
3. Print with `{:?}` and `{:#?}`.
4. Call `dbg!(airspeed_knots);`.
5. Test with:
   ```bash
   cd tasks/02_types_and_math/task_19_debug_printing
   cargo run
   ```

---

## 💡 Hints (Only open if you get stuck!)

<details>
<summary><b>🔍 Hint: Debug placeholders</b></summary>

```rust
println!("Standard Debug: {:?}", flight_location);
println!("Pretty Debug:\n{:#?}", flight_location);
dbg!(airspeed_knots);
```
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/02_types_and_math/task_19_debug_printing/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_19_debug_printing/solution/explain.md)
