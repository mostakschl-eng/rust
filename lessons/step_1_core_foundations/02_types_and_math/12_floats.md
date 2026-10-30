# Lesson 12: Decimal Numbers (Floating Points: `f32` and `f64`)

Welcome to Lesson 12! 

In Lesson 11, we looked at whole numbers. But in the real world, many things have fractions:
* The price of a coffee: `$3.75`
* Body temperature: `98.6` degrees Fahrenheit
* GPS latitude & longitude coordinates: `23.8103, 90.4125`

A **floating-point number** (or **float**) is a representation for approximate real-number arithmetic. Rust uses binary floats; they are not exact decimal-number containers.

---

## 1. Rust's Two Floating-Point Types

Rust has two sizes of decimal boxes:

| Type | Name | Precision | When to Use |
|---|---|---|---|
| `f32` | Single Precision | ~6-7 digits of accuracy | 3D graphics, games, embedded audio chips where memory is tight |
| **`f64`** | Double Precision | **~15-17 digits of accuracy** | **Rust's Default!** Useful for approximate measurements and scientific calculations |

> **Rust Default Rule:**  
> Whenever you type a decimal number like `let price = 19.99;`, Rust defaults to **`f64`** if the surrounding code does not require another float type. Both float types use finite binary precision, so many decimal fractions are approximations.

---

## 2. Code Example: Working with Decimals

```rust
fn main() {
    let coffee_price: f64 = 3.50;
    let tax_rate: f64 = 0.08; // 8% sales tax
    let tax_amount = coffee_price * tax_rate;
    let total = coffee_price + tax_amount;

    println!("Coffee:      ${}", coffee_price);
    println!("Tax:         ${}", tax_amount);
    println!("Total Due:   ${}", total);
}
```

### The Output on Screen:
```text
Coffee:      $3.5
Tax:         $0.28
Total Due:   $3.7800000000000002
```

---

The extra digits in the total demonstrate binary approximation. They are not a bug in println!; Lesson 18 will format the display.

## 3. The Crucial Trap: Integer Division vs Float Division

Look closely at this difference—every beginner gets caught by this:

### Case A: Integer Division (Whole Numbers)
```rust
let result = 5 / 2; // Both are integers!
println!("{}", result); // Prints 2, NOT 2.5!
```
* **Why:** In integer division, the computer **throws away the fractional remainder**. It only tells you how many whole times `2` fits into `5` (which is `2`).

### Case B: Floating-Point Division (Decimals)
```rust
let result = 5.0 / 2.0; // Both have decimals!
println!("{}", result); // Prints 2.5!
```
* **Rule:** If you want a decimal answer, both operands must have compatible floating-point types: write `5.0 / 2.0`, not `5.0 / 2`. Convert stored integers before division (Lesson 16).

---

## 4. You Cannot Mix Integers and Floats Directly!

In Rust, the compiler will never secretly convert numbers behind your back:

```rust
let items = 3;       // integer (i32)
let price = 2.50;    // float (f64)
let total = items * price; // ❌ ERROR!
```
* **What Rust says:** `error: cannot multiply 'i32' by '{float}'`.
* **Why Rust does this:** In languages like C, auto-converting between floats and integers causes silent rounding bugs that lose money. In Rust, you must be explicit! *(In Lesson 16, we will learn how to make explicit numeric casts with `as f64`; some casts lose precision).*

---

## 5. Summary Checklist
- [x] Decimals are called floats in Rust.
- [x] `f64` is the default decimal type with high precision.
- [x] `5 / 2` equals `2`, but `5.0 / 2.0` equals `2.5`.
- [x] You cannot multiply an integer directly with a float without explicit conversion.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/step_1_core_foundations/02_types_and_math/task_12_floating_points/TASK.md`](../../../tasks/step_1_core_foundations/02_types_and_math/task_12_floating_points/TASK.md)**.
1. Read the task requirements.
2. Build a gas station fuel pump calculator using `f64` variables.
3. Run `cargo run` and inspect the approximate fuel total!

## Precision pitfall

Binary floating-point values approximate many decimal fractions. They are useful for this arithmetic demonstration, but not an exact-money guarantee. A real billing system needs an appropriate representation and an explicit rounding rule.
