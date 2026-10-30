# Lesson 12: Decimal Numbers (Floating Points: `f32` and `f64`)

Welcome to Lesson 12! 

In Lesson 11, we looked at whole numbers. But in the real world, many things have fractions:
* The price of a coffee: `$3.75`
* Body temperature: `98.6` degrees Fahrenheit
* GPS latitude & longitude coordinates: `23.8103, 90.4125`

In computer science, decimal numbers are called **Floating-Point Numbers** (or **Floats**).

---

## 1. Rust's Two Floating-Point Types

Rust has two sizes of decimal boxes:

| Type | Name | Precision | When to Use |
|---|---|---|---|
| `f32` | Single Precision | ~6-7 digits of accuracy | 3D graphics, games, embedded audio chips where memory is tight |
| **`f64`** | Double Precision | **~15-17 digits of accuracy** | **Rust's Default!** Perfect for science, physics, money calculations |

> **Rust Default Rule:**  
> Whenever you type a decimal number like `let price = 19.99;`, Rust automatically makes it an **`f64`**. Modern computers process `f64` at lightning speed with maximum precision.

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
Total Due:   $3.78
```

---

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
* **Rule:** If you want a decimal answer, at least one of the numbers **must have a decimal point** (like `5.0`).

---

## 4. You Cannot Mix Integers and Floats Directly!

In Rust, the compiler will never secretly convert numbers behind your back:

```rust
let items = 3;       // integer (i32)
let price = 2.50;    // float (f64)
let total = items * price; // ❌ ERROR!
```
* **What Rust says:** `error: cannot multiply 'i32' by '{float}'`.
* **Why Rust does this:** In languages like C, auto-converting between floats and integers causes silent rounding bugs that lose money. In Rust, you must be explicit! *(In Lesson 16, we will learn how to convert between them safely with `as f64`).*

---

## 5. Summary Checklist
- [x] Decimals are called floats in Rust.
- [x] `f64` is the default decimal type with high precision.
- [x] `5 / 2` equals `2`, but `5.0 / 2.0` equals `2.5`.
- [x] You cannot multiply an integer directly with a float without explicit conversion.

---

## 🛠️ Ready for Practice!

👉 Open **[`tasks/02_types_and_math/task_12_floating_points/TASK.md`](file:///d:/rust-learning/tasks/02_types_and_math/task_12_floating_points/TASK.md)**.
1. Read the task requirements.
2. Build a gas station fuel pump calculator using `f64` variables.
3. Run `cargo run` and calculate the exact fuel total!
