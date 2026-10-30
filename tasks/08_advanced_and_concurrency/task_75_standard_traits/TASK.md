# Task 75: Standard Library Traits (`Display` & `Default`)

## 🎯 What is Your Task?
Implement Rust's standard library traits to create clean user formatting and automatic configuration defaults:

1. **Custom `Display` for `Money`**:
   - Create a struct:
     ```rust
     struct Money {
         cents: u64,
     }
     ```
   - Manually implement `std::fmt::Display` for `Money` so that:
     - Dollars = `self.cents / 100`
     - Cents = `self.cents % 100`
     - Writes to formatter formatted as `$<dollars>.<cents:02>` (e.g. `2450` cents becomes `$24.50`).

2. **Derive `Default` & `Debug` for `AudioSettings`**:
   - Create a struct:
     ```rust
     #[derive(Debug, Default)]
     struct AudioSettings {
         volume: u8,
         enable_surround: bool,
         sample_rate: u32,
     }
     ```

3. **In `main()`**:
   - Create `let item_price = Money { cents: 2450 };`
   - Print using `{}`: `"Item Price: {}"`
   - Create `let default_audio = AudioSettings::default();`
   - Print using `{:?}`: `"Default Audio: {:?}"`
   - Create `let custom_audio = AudioSettings { volume: 85, ..Default::default() };`
   - Print using `{:?}`: `"Custom Audio:  {:?}"`

---

## 📋 Expected Output on Your Screen
```text
=== SYSTEM CONFIG & PRICING ===
Item Price:     $24.50
Default Audio:  AudioSettings { volume: 0, enable_surround: false, sample_rate: 0 }
Custom Audio:   AudioSettings { volume: 85, enable_surround: false, sample_rate: 0 }
===============================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_75_standard_traits/src/main.rs).
2. Bring `std::fmt` into scope: `use std::fmt;`.
3. Implement `fmt::Display for Money`.
4. Derive `Debug, Default` on `AudioSettings`.
5. Run your project:
   ```bash
   cd tasks/08_advanced_and_concurrency/task_75_standard_traits
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_75_standard_traits/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_75_standard_traits/solution/explain.md)

