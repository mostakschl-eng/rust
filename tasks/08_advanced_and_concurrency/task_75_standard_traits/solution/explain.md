# Detailed Explanation: Task 75 (Standard Library Traits)

---

### 1. Manual `Display` Implementation
```rust
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.cents / 100;
        let cents = self.cents % 100;
        write!(f, "${}.{:02}", dollars, cents)
    }
}
```
* **Why not `#[derive(Display)]`?**
  Rust does not derive `Display` because formatting depends entirely on user-facing design choices (e.g. should currency display as `$24.50`, `24.50 USD`, or `€24,50`?).
* **`write!(f, ...)`**: Writes into the formatter buffer `f`.
* **`{:02}` formatting**: Ensures that numbers less than 10 are padded with a leading zero (e.g., 5 cents formats as `.05`, not `.5`).

---

### 2. Deriving `Default` and Struct Update Syntax
```rust
#[derive(Debug, Default)]
struct AudioSettings {
    volume: u8,
    enable_surround: bool,
    sample_rate: u32,
}
```
* **`Default` Trait**:
  Automatically sets:
  - Numbers (`u8`, `u32`) to `0`
  - Booleans (`bool`) to `false`
  - Strings to empty `""`
* **Struct Update Syntax**:
  ```rust
  let custom_audio = AudioSettings {
      volume: 85,
      ..Default::default()
  };
  ```
  Only the `volume` field is explicitly set; the remaining fields are filled from `Default::default()`.

