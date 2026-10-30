# Task 43: Graphic Canvas & Security Token (Tuple & Unit Structs)

## 🎯 What is Your Task?
Write a Rust program to implement strong type-safe graphics primitives and a security marker:
1. Define a tuple struct `Color(u8, u8, u8)`.
2. Define a tuple struct `Point3D(i32, i32, i32)`.
3. Define a unit-like struct `SecurityKey`.
4. In `main()`:
   - Create `sky_blue = Color(135, 206, 235)`.
   - Create `cursor = Point3D(100, 250, -50)`.
   - Create `token = SecurityKey`.
   - Access and print RGB channels using `.0`, `.1`, `.2`.
   - Unpack/destructure `cursor` using `let Point3D(x, y, z) = cursor;` and print coordinates.
   - Print token validation message.

---

## 📋 Expected Output on Your Screen
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

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](src/main.rs).
2. Declare tuple structs with `(Type, Type, ...);` ending with `;`.
3. Declare `struct SecurityKey;`.
4. Test:
   ```bash
   cd tasks/step_1_core_foundations/05_structs_and_enums/task_43_tuple_and_unit_structs
   cargo run
   ```

---

## 💡 Hints

<details>
<summary>Hint 1: choose the approach</summary>

Use a different tuple struct name for each domain, even when fields have similar types.

</details>

<details>
<summary>Hint 2: connect the syntax</summary>

Access Color channels with `.0`–`.2`, destructure Point3D with its type name, and create a fieldless SecurityKey value.

</details>

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](solution/explain.md)


## Watch out

The marker has no authentication behavior. A printed validation message is pretend; anyone in this module can construct SecurityKey.

The starter intentionally does not produce the expected output yet. Complete its TODOs before comparing results.

## Marker scope

The SecurityKey message is a demonstration. Constructing a zero-sized marker performs no real security validation.
