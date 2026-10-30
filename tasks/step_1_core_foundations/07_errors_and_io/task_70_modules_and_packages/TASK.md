# Task 70: Fortress Security & Telemetry Modules (mod, pub, use)

## 🎯 What is Your Task?
Write a Rust program demonstrating clean modular architecture and privacy boundaries:
1. Define a module `security`:
   - A private helper function `hash_token(input: &str) -> String` that returns `format!("HASH_{}", input)` as a visibility demonstration only, not cryptography.
   - A public function `pub fn verify_credentials(user: &str, token: &str) -> bool` that checks user `"Admin"` and the toy transformed token `"HASH_SECRET_99"`.
2. Define a module `telemetry`:
   - A public struct `pub struct Sensor` with public fields `pub id: u32`, `pub label: String`, and private field `calibration_offset: f64`.
   - A public constructor `pub fn new(id: u32, label: &str) -> Sensor` that sets calibration_offset to `1.32`.
   - A public method `pub fn measure(&self) -> f64` that returns `100.0 + self.calibration_offset`.
3. In `main()`:
   - Bring the public items into scope with `use`.
   - Verify credentials for `"Admin"` with token `"SECRET_99"`.
   - Instantiate a Sensor and call `.measure()`.

---

## 📋 Expected Output on Your Screen
```text
=== FORTRESS MODULAR SYSTEM ===
[1. Testing Security Module]
Credentials verified: true

[2. Testing Telemetry Module]
Sensor #401 (Pressure Gauge): Reading = 101.32 kPa
Modular boundaries enforced cleanly! 🛡️
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Declare modules with `mod`, items with `pub`, and import with `use`.
3. Test:
   ```bash
   cd tasks/step_1_core_foundations/07_errors_and_io/task_70_modules_and_packages
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Hints (Open One at a Time)

<details>
<summary>Hint 1: Choose the approach</summary>

Put the token helper inside security and expose only the requested verifier.

</details>

<details>
<summary>Hint 2: Connect the pieces</summary>

Keep the calibration field private, initialize it in Sensor::new, and import the verifier and Sensor for main.

</details>

**Before running:** The toy hash_token only prefixes text and is not a password hash or secure authentication. Public Sensor fields are still part of its API, so changing them can break callers.
