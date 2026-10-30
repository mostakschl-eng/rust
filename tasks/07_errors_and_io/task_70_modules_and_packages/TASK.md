# Task 70: Fortress Security & Telemetry Modules (mod, pub, use)

## 🎯 What is Your Task?
Write a Rust program demonstrating clean modular architecture and privacy boundaries:
1. Define a module `security`:
   - A private helper function `hash_token(input: &str) -> String`.
   - A public function `pub fn verify_credentials(user: &str, token: &str) -> bool`.
2. Define a module `telemetry`:
   - A public struct `pub struct Sensor` with public fields `pub id: u32`, `pub label: String`, and private field `calibration_offset: f64`.
   - A public constructor `pub fn new(id: u32, label: &str) -> Sensor`.
   - A public method `pub fn measure(&self) -> f64`.
3. In `main()`:
   - Bring the public items into scope with `use`.
   - Verify credentials for `"Admin"`.
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

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_70_modules_and_packages/src/main.rs).
2. Declare modules with `mod`, items with `pub`, and import with `use`.
3. Test:
   ```bash
   cd tasks/07_errors_and_io/task_70_modules_and_packages
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/07_errors_and_io/task_70_modules_and_packages/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/07_errors_and_io/task_70_modules_and_packages/solution/explain.md)

