# Solution Explanation: Task 70 (Modules & Packages)

Let's dissect how modules and visibility boundaries enforce clean software architecture!

---

## 🔍 The Solution Code

```rust
mod security {
    fn hash_token(input: &str) -> String { ... } // Private!
    pub fn verify_credentials(...) -> bool { ... } // Public!
}

mod telemetry {
    pub struct Sensor {
        pub id: u32,
        pub label: String,
        calibration_offset: f64, // Private field!
    }
    impl Sensor {
        pub fn new(...) -> Self { ... }
        pub fn measure(&self) -> f64 { ... }
    }
}
```

---

## 🧩 Key Takeaways

1. **Private by Default:**
   * `hash_token` is internal to `security` and cannot be called from `main`.
   * `calibration_offset` is internal to `telemetry` and cannot be read directly or modified by outside code.
2. **Public Interfaces (`pub`):**
   * Code interacts with `Sensor` exclusively through the public constructor `Sensor::new()` and public method `sensor.measure()`.
   * This guarantees that internal implementation details can change in the future without breaking callers!

