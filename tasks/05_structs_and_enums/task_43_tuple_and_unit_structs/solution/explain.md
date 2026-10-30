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

