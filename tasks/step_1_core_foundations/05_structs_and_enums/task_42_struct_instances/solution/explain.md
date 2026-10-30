# Solution Explanation: Task 42 (Struct Update Syntax)

Let's dissect how the struct update syntax saves boilerplate and prevents repetitive initialization!

---

## 🔍 The Solution Code

```rust
#[derive(Debug)]
struct DeviceSpec {
    model: String,
    storage_gb: u32,
    ram_gb: u32,
    has_5g: bool,
}

fn main() {
    println!("=== PHONE HARDWARE CONFIGURATOR ===");

    let phone_base = DeviceSpec {
        model: String::from("Pixel Standard"),
        storage_gb: 128,
        ram_gb: 8,
        has_5g: true,
    };

    println!("Base Model:\n{:?}", phone_base);

    let phone_pro = DeviceSpec {
        model: String::from("Pixel Pro"),
        storage_gb: 256,
        ..phone_base
    };

    println!("\nPro Model (Upgraded Storage & Model via ..):");
    println!("{:?}", phone_pro);

    println!("===================================");
}
```

---

## 🧩 Key Takeaways

1. **`..phone_base` Syntax:**
   * Automatically copies every field not explicitly defined above it (`ram_gb` and `has_5g`).
   * The `..` expression must always be the final entry inside the braces.
2. **Avoiding Moved Strings:**
   * Because we explicitly supplied a new `model` string (`"Pixel Pro"`), `phone_base.model` was NOT moved!
   * The remaining fields (`ram_gb: 8` and `has_5g: true`) implement `Copy`, so `phone_base` remains completely valid.

## Everyday picture and symbols

Think of filling a second form by reusing unchanged entries. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `..phone_base` | Supply unspecified fields from the base |
| `Copy` | Trait allowing implicit copies |
| `String` | Owned text that moves unless explicitly replaced |

## Common pitfall

Update syntax can move non-Copy fields; it is not a deep clone of every field.

## Expected output

```text
=== PHONE HARDWARE CONFIGURATOR ===
Base Model:
DeviceSpec { model: "Pixel Standard", storage_gb: 128, ram_gb: 8, has_5g: true }

Pro Model (Upgraded Storage & Model via ..):
DeviceSpec { model: "Pixel Pro", storage_gb: 256, ram_gb: 8, has_5g: true }
===================================
```
