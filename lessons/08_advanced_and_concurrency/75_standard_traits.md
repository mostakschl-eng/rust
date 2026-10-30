# Lesson 75: Essential Standard Library Traits (`Display`, `Debug`, `Clone`, `Default`)

Rust's standard library is packed with dozens of built-in traits that define how types behave in everyday operations.  
Mastering these 4 essential traits will make your custom types feel like first-class citizens in the Rust ecosystem! 🌟

---

## 1. The Four Horsemen of Daily Rust

| Trait | What It Enables | Common Syntax | How to Implement |
|---|---|:---:|---|
| **`Debug`** | Developer output formatting | `{:?}` or `{:#?}` | `#[derive(Debug)]` |
| **`Display`** | End-user clean string formatting | `{}` | Manual `impl fmt::Display` |
| **`Clone`** | Explicit deep copying of data | `.clone()` | `#[derive(Clone)]` |
| **`Default`** | Standard default values | `Type::default()` | `#[derive(Default)]` |

---

## 2. `Debug` vs `Display`: What's the Difference?

* **`Debug` (`{:?}`):** Meant for **programmers**. Shows exact field names, raw types, and internal struct details for troubleshooting.
* **`Display` (`{}`):** Meant for **end users**. Clean, beautifully formatted text.

Because user-facing formatting depends on human aesthetics, Rust does **not** allow deriving `Display` automatically. You write it manually using `std::fmt`:

```rust
use std::fmt;

struct Money {
    cents: u64,
}

// Implement Display for beautiful user formatting:
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.cents / 100;
        let cents = self.cents % 100;
        write!(f, "${}.{:02}", dollars, cents)
    }
}

fn main() {
    let price = Money { cents: 1999 };

    // Uses our custom Display implementation:
    println!("Total cost: {}", price);
}
```

### 📋 Expected Terminal Output:
```text
Total cost: $19.99
```

Look how clean and professional `$19.99` is!

---

## 3. The `Default` Trait: Standard Starting Values

Have you ever created a configuration struct with 10 fields, where 8 of them should just use sensible defaults?  
Implement or derive the **`Default` trait**:

```rust
#[derive(Debug, Default)]
struct AudioSettings {
    volume: u8,
    enable_surround: bool,
    sample_rate: u32,
}

fn main() {
    // 1. Create standard defaults:
    let default_audio = AudioSettings::default();
    println!("Default Settings: {:?}", default_audio);

    // 2. Combine with struct update syntax!
    let loud_audio = AudioSettings {
        volume: 100, // Custom volume
        ..Default::default() // Default for the rest!
    };
    println!("Custom Audio:     {:?}", loud_audio);
}
```

### 📋 Expected Terminal Output:
```text
Default Settings: AudioSettings { volume: 0, enable_surround: false, sample_rate: 0 }
Custom Audio:     AudioSettings { volume: 100, enable_surround: false, sample_rate: 0 }
```

---

## 4. Derived vs Manual Trait Implementations

Most common standard traits can be automatically derived in a single line:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Server {
    ip: String,
    port: u16,
}
```

This single line saves you from writing over 50 lines of boilerplate code!

---

## 5. Quick Recap Checklist
- [x] `Debug` (`{:?}`) is for developer inspection; derive it with `#[derive(Debug)]`.
- [x] `Display` (`{}`) is for end users; implement it manually with `impl fmt::Display`.
- [x] `Clone` creates explicit duplicates via `.clone()`.
- [x] `Default` provides fallback defaults via `Type::default()`.

---

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/08_advanced_and_concurrency/task_75_standard_traits/TASK.md`](file:///d:/rust-learning/tasks/08_advanced_and_concurrency/task_75_standard_traits/TASK.md).
* Next lesson: [Lesson 76: Closures: Anonymous Inline Functions (`\|x\| x + 1`)](file:///d:/rust-learning/lessons/08_advanced_and_concurrency/76_closures.md).
