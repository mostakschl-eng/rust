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
   * Callers use `Sensor::new()` to construct a sensor and `sensor.measure()` to read the value that uses its private calibration. They can also read or change the public `id` and `label` fields directly.
   * Keeping calibration private allows that internal representation to change while preserving the public API. The public id and label fields are still caller-visible contracts.


## Reading the Result Carefully

A restaurant exposes its order counter while keeping recipes in the kitchen. Visibility is an API boundary, not cryptographic protection.

| Symbol or term | Meaning |
|---|---|
| `mod` | module |
| `pub` | public visibility |
| `use` | local name import |
| `::` | path separator |

The toy hash_token only prefixes text and is not a password hash or secure authentication. Public Sensor fields are still part of its API, so changing them can break callers.

The complete runnable program is in `solution.rs`; snippets above may focus on only one part. Compare its output with the example in TASK.md.
