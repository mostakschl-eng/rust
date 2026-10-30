# Solution Explanation: Task 43 (Tuple Structs & Unit Structs)

Let's dissect how tuple structs and unit structs provide lightweight type safety!

---

## 🔍 The Solution Code

```rust
struct Color(u8, u8, u8);
struct Point3D(i32, i32, i32);
struct SecurityKey;

fn main() {
    println!("=== GRAPHICS CANVAS & TOKEN SYSTEM ===");

    let sky_blue = Color(135, 206, 235);
    println!("Sky Blue Color:");
    println!("  Red:   {}", sky_blue.0);
    println!("  Green: {}", sky_blue.1);
    println!("  Blue:  {}", sky_blue.2);

    let cursor = Point3D(100, 250, -50);
    let Point3D(x, y, z) = cursor;
    println!("\n3D Cursor Position:");
    println!("  Coordinates: X={}, Y={}, Z={}", x, y, z);

    let _token = SecurityKey;
    println!("\nSecurity Token:");
    println!("  SecurityKey validated! (0-byte marker)");

    println!("======================================");
}
```

---

## 🧩 Key Takeaways

1. **Tuple Struct Declaration:**
   * Uses parentheses `struct Color(u8, u8, u8);` with a required trailing semicolon.
   * Gives a dedicated nominal type to a tuple, preventing mixing up colors with spatial coordinates.
2. **Accessing & Destructuring:**
   * Field access: `sky_blue.0`, `sky_blue.1`.
   * Destructuring: `let Point3D(x, y, z) = cursor;`.
3. **Unit-Like Structs:**
   * `struct SecurityKey;` has no fields and consumes 0 bytes of RAM.
   * The prefix underscore `_token` silences unused variable warnings.

## Everyday picture and symbols

Think of different labeled boxes for colors and coordinates. The code turns that picture into explicit values and operations.

| Symbol or syntax | Meaning |
|---|---|
| `struct Color(u8, u8, u8);` | Define a tuple struct |
| `Color(...)` | Construct that named type |
| `let Point3D(x, y, z)` | Destructure its fields |
| `struct SecurityKey;` | Define a fieldless marker |

## Common pitfall

The marker has no authentication behavior. A printed validation message is pretend; anyone in this module can construct SecurityKey.

## Expected output

```text
=== GRAPHICS CANVAS & TOKEN SYSTEM ===
Sky Blue Color:
  Red:   135
  Green: 206
  Blue:  235

3D Cursor Position:
  Coordinates: X=100, Y=250, Z=-50

Security Token:
  SecurityKey validated! (0-byte marker)
======================================
```
