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
=================================
```

---

The `dbg!` call writes a separate diagnostic to **stderr**. The file path and location depend on where the code runs, and its order relative to stdout can vary:

```text
[YOUR_FILE:LINE:COLUMN] airspeed_knots = 505
```

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Declare `flight_location = ("Flight 707", 32.7767, -96.7970, 35000);`.
3. Print with `{:?}` and `{:#?}`.
4. Call `dbg!(airspeed_knots);`.
5. Test with:
   ```bash
   cd tasks/step_1_core_foundations/02_types_and_math/task_19_debug_printing
   cargo run
   ```

---

## 💡 Hints (open one at a time)

<details>
<summary>Hint 1: Start with the idea</summary>

The tuple groups four different values. Use Debug rather than ordinary Display for the whole tuple.
</details>

<details>
<summary>Hint 2: Connect it to Rust</summary>

Add # to the Debug formatter for multiline output. dbg! writes to stderr, and its location depends on where you put the call.
</details>

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)

